/* ABI configuration boundary only; solver policy and callbacks live in Rust. */
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <sundials/sundials_context.h>
#include <sundials/sundials_config.h>
#include <sundials/sundials_types.h>
#ifdef MBSUN_WITH_KLU
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <SuiteSparse_config.h>
#include <sunlinsol/sunlinsol_klu.h>
#endif
#if SUNDIALS_MPI_ENABLED
#error ModernBASIC requires the qualified serial SUNDIALS build
#endif
#if SUNDIALS_VERSION_MAJOR != 7 || SUNDIALS_VERSION_MINOR != 1 || SUNDIALS_VERSION_PATCH != 1
#error Unqualified SUNDIALS version
#endif
_Static_assert(sizeof(sunrealtype)==sizeof(double),"double precision required");
_Static_assert(sizeof(sunindextype)==sizeof(int64_t),"int64 indices required");
const char *mbsun_abi(void) {
#ifdef MBSUN_WITH_KLU
return "ModernBASIC/SUNDIALS/6;7.1.1;double;int64;serial;CVODES+IDA+SPGMR+PRECONDITIONER+CONSISTENTIC+JACOBIAN+CVODEJACOBIAN+CVODEJTIMES+CVODEROOTS+CVODESFSA+KLU+MEMCAP64M";
#else
return "ModernBASIC/SUNDIALS/6;7.1.1;double;int64;serial;CVODES+IDA+SPGMR+PRECONDITIONER+CONSISTENTIC+JACOBIAN+CVODEJACOBIAN+CVODEJTIMES+CVODEROOTS+CVODESFSA";
#endif
}
int mbsun_context(void **context) {return SUNContext_Create(SUN_COMM_NULL,(SUNContext*)context);}

#ifdef MBSUN_WITH_KLU
/* SuiteSparseConfig's static Debian build has one OpenMP timing reference.
 * SUNDIALS and KLU run serially here; provide the monotonic wall clock without
 * pulling a threaded runtime into the isolated provider. */
double omp_get_wtime(void) {
  struct timespec value;
  if (clock_gettime(CLOCK_MONOTONIC, &value) != 0) return 0.0;
  return (double)value.tv_sec + (double)value.tv_nsec * 1e-9;
}

/* KLU allocates its factors through SuiteSparse's process-wide allocator
 * hooks. Install a transparent aligned header wrapper once, then enable a
 * thread-local byte budget only while Rust is running one synchronous KLU
 * solve. The provider contains its own static SuiteSparse copy, separate from
 * the runtime's other sparse backends. */
typedef union {
  max_align_t alignment;
  struct { size_t bytes; uint64_t scope; } value;
} mbsun_klu_header;

typedef struct {
  size_t limit, current, peak;
  uint64_t serial, scope;
  int active, exceeded;
} mbsun_klu_scope;

static void *(*mbsun_base_malloc)(size_t);
static void *(*mbsun_base_calloc)(size_t, size_t);
static void *(*mbsun_base_realloc)(void *, size_t);
static void (*mbsun_base_free)(void *);
static int mbsun_allocator_installed;
static _Thread_local mbsun_klu_scope mbsun_scope;

static void *mbsun_cap_malloc(size_t bytes);
static void *mbsun_cap_calloc(size_t count, size_t bytes);
static void *mbsun_cap_realloc(void *pointer, size_t bytes);
static void mbsun_cap_free(void *pointer);

static int mbsun_cap_allowed(size_t size, size_t current) {
  if (!mbsun_scope.active) return 1;
  if (size > mbsun_scope.limit || current > mbsun_scope.limit - size) {
    mbsun_scope.exceeded = 1;
    return 0;
  }
  return 1;
}

static void mbsun_cap_record(size_t size) {
  if (!mbsun_scope.active) return;
  mbsun_scope.current += size;
  if (mbsun_scope.current > mbsun_scope.peak)
    mbsun_scope.peak = mbsun_scope.current;
}

static void *mbsun_cap_malloc(size_t bytes) {
  size_t total;
  mbsun_klu_header *header;
  if (bytes > SIZE_MAX - sizeof(*header)) return NULL;
  total = sizeof(*header) + bytes;
  if (!mbsun_cap_allowed(total, mbsun_scope.current)) return NULL;
  header = (mbsun_klu_header *)mbsun_base_malloc(total);
  if (header == NULL) return NULL;
  header->value.bytes = total;
  header->value.scope = mbsun_scope.active ? mbsun_scope.scope : 0;
  mbsun_cap_record(total);
  return (void *)(header + 1);
}

static void *mbsun_cap_calloc(size_t count, size_t bytes) {
  size_t total;
  void *pointer;
  if (count != 0 && bytes > SIZE_MAX / count) return NULL;
  total = count * bytes;
  pointer = mbsun_cap_malloc(total);
  if (pointer != NULL) memset(pointer, 0, total);
  return pointer;
}

static void *mbsun_cap_realloc(void *pointer, size_t bytes) {
  mbsun_klu_header *old_header, *new_header;
  size_t old_total, new_total, base_current;
  uint64_t old_scope;
  if (pointer == NULL) return mbsun_cap_malloc(bytes);
  if (bytes == 0) {
    mbsun_cap_free(pointer);
    return NULL;
  }
  if (bytes > SIZE_MAX - sizeof(*old_header)) return NULL;
  old_header = ((mbsun_klu_header *)pointer) - 1;
  old_total = old_header->value.bytes;
  old_scope = old_header->value.scope;
  new_total = sizeof(*old_header) + bytes;
  base_current = mbsun_scope.current;
  if (mbsun_scope.active && old_scope == mbsun_scope.scope)
    base_current -= old_total;
  if (!mbsun_cap_allowed(new_total, base_current)) return NULL;
  new_header = (mbsun_klu_header *)mbsun_base_realloc(old_header, new_total);
  if (new_header == NULL) return NULL;
  new_header->value.bytes = new_total;
  new_header->value.scope = mbsun_scope.active ? mbsun_scope.scope : 0;
  if (mbsun_scope.active) {
    mbsun_scope.current = base_current + new_total;
    if (mbsun_scope.current > mbsun_scope.peak)
      mbsun_scope.peak = mbsun_scope.current;
  }
  return (void *)(new_header + 1);
}

static void mbsun_cap_free(void *pointer) {
  mbsun_klu_header *header;
  if (pointer == NULL) return;
  header = ((mbsun_klu_header *)pointer) - 1;
  if (mbsun_scope.active && header->value.scope == mbsun_scope.scope) {
    if (header->value.bytes <= mbsun_scope.current)
      mbsun_scope.current -= header->value.bytes;
    else
      mbsun_scope.current = 0;
  }
  mbsun_base_free(header);
}

int mbsun_klu_memory_cap_install(void) {
  if (mbsun_allocator_installed) return 0;
  mbsun_base_malloc = SuiteSparse_config_malloc_func_get();
  mbsun_base_calloc = SuiteSparse_config_calloc_func_get();
  mbsun_base_realloc = SuiteSparse_config_realloc_func_get();
  mbsun_base_free = SuiteSparse_config_free_func_get();
  if (mbsun_base_malloc == NULL) mbsun_base_malloc = malloc;
  if (mbsun_base_calloc == NULL) mbsun_base_calloc = calloc;
  if (mbsun_base_realloc == NULL) mbsun_base_realloc = realloc;
  if (mbsun_base_free == NULL) mbsun_base_free = free;
  SuiteSparse_config_malloc_func_set(mbsun_cap_malloc);
  SuiteSparse_config_calloc_func_set(mbsun_cap_calloc);
  SuiteSparse_config_realloc_func_set(mbsun_cap_realloc);
  SuiteSparse_config_free_func_set(mbsun_cap_free);
  mbsun_allocator_installed = 1;
  return 0;
}

int mbsun_klu_memory_cap_begin(size_t limit) {
  if (!mbsun_allocator_installed || mbsun_scope.active || limit < sizeof(mbsun_klu_header))
    return -1;
  mbsun_scope.serial++;
  if (mbsun_scope.serial == 0) mbsun_scope.serial++;
  mbsun_scope.scope = mbsun_scope.serial;
  mbsun_scope.limit = limit;
  mbsun_scope.current = 0;
  mbsun_scope.peak = 0;
  mbsun_scope.exceeded = 0;
  mbsun_scope.active = 1;
  return 0;
}

int mbsun_klu_memory_cap_status(size_t *peak, size_t *current, int *exceeded) {
  if (!mbsun_scope.active || peak == NULL || current == NULL || exceeded == NULL)
    return -1;
  *peak = mbsun_scope.peak;
  *current = mbsun_scope.current;
  *exceeded = mbsun_scope.exceeded;
  return 0;
}

int mbsun_klu_memory_cap_end(size_t *peak, size_t *current, int *exceeded) {
  int status = mbsun_klu_memory_cap_status(peak, current, exceeded);
  if (status == 0) mbsun_scope.active = 0;
  return status;
}
#endif
