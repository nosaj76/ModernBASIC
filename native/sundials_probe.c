#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>
#include <cvode/cvode.h>
#include <cvode/cvode_ls.h>
#include <nvector/nvector_serial.h>
#include <sunmatrix/sunmatrix_dense.h>
#include <sunlinsol/sunlinsol_dense.h>
extern const char *mbsun_abi(void);
extern int mbsun_context(SUNContext *);
#define OK(call) do { int status=(call); if(status<0) { fprintf(stderr,"%s: %d\n",#call,status); exit(2); } } while(0)
static int rhs(sunrealtype t,N_Vector y,N_Vector dy,void *user) {
 (void)t; double *a=N_VGetArrayPointer(y),*b=N_VGetArrayPointer(dy);
 if ((intptr_t)user==0) b[0]=-a[0];
 else {double x=.04*a[0]-1e4*a[1]*a[2],z=3e7*a[1]*a[1];b[0]=-x;b[1]=x-z;b[2]=z;}
 return 0;
}
static void solve(int method,int problem) {
 SUNContext context=NULL; OK(mbsun_context(&context));
 int n=problem?3:1; N_Vector y=N_VNew_Serial(n,context),atol=N_VNew_Serial(n,context);
 if(!y||!atol) exit(3);
 for(int i=0;i<n;i++){NV_Ith_S(y,i)=i?0:1;NV_Ith_S(atol,i)=problem&&i==1?1e-15:1e-12;}
 SUNMatrix matrix=SUNDenseMatrix(n,n,context); SUNLinearSolver linear=SUNLinSol_Dense(y,matrix,context);
 void *memory=CVodeCreate(method,context);if(!matrix||!linear||!memory)exit(3);
 OK(CVodeInit(memory,rhs,0,y));OK(CVodeSetUserData(memory,(void*)(intptr_t)problem));
 OK(CVodeSVtolerances(memory,1e-9,atol));OK(CVodeSetLinearSolver(memory,linear,matrix));OK(CVodeSetMaxNumSteps(memory,100000));
 double ordinary[]={.1,1,3},stiff[]={1e-6,.001,.1,1,10,40};int count=problem?6:3;
 for(int k=0;k<count;k++) {double requested=problem?stiff[k]:ordinary[k],returned=0;OK(CVode(memory,requested,y,&returned,CV_NORMAL));
 if(returned!=requested){exit(4);}
 double sum=0;for(int i=0;i<n;i++){if(!isfinite(NV_Ith_S(y,i)))exit(4);sum+=NV_Ith_S(y,i);}
 if(problem){if(fabs(sum-1)>1e-8)exit(4);}else if(fabs(NV_Ith_S(y,0)-exp(-returned))>1e-7)exit(4);
 printf("{\"method\":%d,\"problem\":%d,\"t\":%.17g,\"state\":[",method,problem,returned);
 for(int i=0;i<n;i++){printf("%s%.17g",i?",":"",NV_Ith_S(y,i));}
 printf("]}\n");}
 CVodeFree(&memory);SUNLinSolFree(linear);SUNMatDestroy(matrix);N_VDestroy(atol);N_VDestroy(y);SUNContext_Free(&context);
}
int main(void){if(strcmp(mbsun_abi(),"ModernBASIC/SUNDIALS/6;7.1.1;double;int64;serial;CVODE+IDA+SPGMR+PRECONDITIONER+CONSISTENTIC+JACOBIAN+CVODEJACOBIAN+CVODEJTIMES+CVODEROOTS"))return 5;solve(CV_BDF,0);solve(CV_ADAMS,0);solve(CV_BDF,1);return 0;}
