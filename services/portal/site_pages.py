"""Public distro pages; report intake and developer workflows live in app.py."""
from html import escape as e
from urllib.parse import parse_qs

PLATFORM_DETAILS = {
    'amd64': ('PC · x86-64', 'For 64-bit Intel and AMD PCs.', 'BIOS or UEFI. Check the release’s tested hardware and boot notes. Secure Boot signing is not included in the current release work.'),
    'arm64': ('ARM64', 'For compatible 64-bit ARM systems.', 'Generic UEFI systems only unless the image’s release notes name your board. Boot firmware, device trees and graphics support are board-specific. An ARM processor alone does not establish compatibility.'),
    'riscv64': ('RISC-V 64', 'For compatible 64-bit RISC-V systems.', 'Requires matching UEFI firmware and board support. Use only hardware covered by the image’s acceptance notes.'),
    'pi5': ('Raspberry Pi 5', 'For Raspberry Pi 5 board support.', 'The existing Pi 5 profile is a package for a compatible Debian-based board OS. A generic ARM64 ISO is not a Pi 5 image. A dedicated boot image has not yet been published.')}


def intro(eyebrow, title, text):
    return f'<div class="page-intro"><p class="eyebrow">{e(eyebrow)}</p><h1>{e(title)}</h1><p class="lead">{e(text)}</p></div>'


def card(platform, downloads):
    title, description, compatibility = PLATFORM_DETAILS[platform]
    item = downloads.get(platform)
    if item:
        name = e(item['filename'])
        action = f'<p class="badge">{e(item["version"])}</p><a class="button" href="/downloads/{name}">Download {"ISO" if name.endswith(".iso") else "image"} ↓</a><p>{item["bytes"] / 1024**3:.2f} GiB · <a href="/checksums/{name}">SHA-256</a></p><p><a href="/releases#{platform}">Release details</a></p>'
    else:
        action = '<p class="badge muted">Awaiting release validation</p><p>No accepted image published yet.</p><a href="/releases">View release status →</a>'
    return f'<article class="card" id="{platform}"><h3>{e(title)}</h3><p>{e(description)}</p>{action}</article>'


def manual_nav(documents, current=''):
    result, group = [], None
    for doc in documents:
        if doc['group'] == 'Project':
            continue
        if doc['group'] != group:
            if group is not None:
                result.append('</ul>')
            group = doc['group']
            result.append('<h3>' + e(group) + '</h3><ul>')
        active = ' aria-current="page"' if doc['slug'] == current else ''
        result.append(f'<li><a href="/manual/{doc["slug"]}"{active}>{e(doc["title"])}</a></li>')
    if group is not None:
        result.append('</ul>')
    return ''.join(result)


def search_form(value=''):
    return f'<form class="manual-search" action="/manual/search" method="get"><label for="manual-query">Search the user manual</label><div><input id="manual-query" name="q" value="{e(value, quote=True)}" maxlength="100" placeholder="Try PRINT, colors, saving, audio…" required><button>Search</button></div></form>'


def public_page(path, query, portal):
    docs = portal.documents
    if path == '/':
        cards = ''.join(card(p, portal.downloads) for p in PLATFORM_DETAILS)
        return 'A Linux distro for creating with BASIC', f'''<section class="hero"><div><p class="eyebrow">A LINUX DISTRO FOR THE CREATOR IN YOU</p><h1>Boot up.<br>Make something.</h1><p class="lead">Meet ModernBASIC: a Debian-based operating system with a native BASIC workspace at its heart. Write a game, draw an idea, compose a melody. Your computer is yours to program.</p><div class="actions"><a class="button" href="/downloads">Get ModernBASIC ↓</a><a href="#intro-film">Watch the 48-second tour ↓</a></div></div><div class="terminal" aria-label="Example BASIC program"><div class="terminal-bar"><span>modernBASIC · hello.bas</span><span>READY.</span></div><pre><span>10</span> FOR I = 1 TO 5\n<span>20</span> PRINT "I MADE THIS!"\n<span>30</span> NEXT I\n<span>40</span> END\n\nRUN\nI MADE THIS!\nI MADE THIS!\nI MADE THIS!\nI MADE THIS!\nI MADE THIS!\n\nREADY. <b>▌</b></pre></div></section>
<div class="facts"><span>Debian Linux foundation</span><span>Native 64-bit BASIC</span><span>Programs, graphics &amp; music</span><span>Built-in offline manual</span></div>
<section id="intro-film" class="intro-film" aria-labelledby="intro-film-title"><div class="film-heading"><div><p class="eyebrow">A LITTLE CODE. A WORLD OF POSSIBILITIES.</p><h2 id="intro-film-title">See what you could make.</h2></div><p>48 seconds · Real BASIC demos<br>Original synth soundtrack</p></div>
<figure class="film-player"><video controls playsinline preload="none" poster="/media/modernbasic-intro-poster.jpg" width="1920" height="1080" aria-label="Introducing ModernBASIC" aria-describedby="film-caption"><source src="/media/modernbasic-intro.mp4" type="video/mp4"><track kind="captions" src="/media/modernbasic-intro.vtt" srclang="en" label="English"><p>Your browser cannot play this video. <a href="/media/modernbasic-intro.mp4">Download the introduction</a> or read the transcript below.</p></video><figcaption id="film-caption">From a first program to moving graphics, music, and Linux underneath. Press play when you’re ready; the film includes music.</figcaption></figure>
<div class="film-followup"><a class="button" href="/start">Make your first program →</a><a href="/manual">Explore the manual →</a><a class="film-download" href="/media/modernbasic-intro.mp4" download>Download the film</a></div>
<details class="film-transcript"><summary>Read the film transcript</summary><ol><li><strong>00:00 — A little code. A world of possibilities.</strong> A Debian-based operating system for creating with BASIC. Original instrumental synth music plays.</li><li><strong>00:08 — Start with a few lines.</strong> A short FOR loop prints “I made this.” three times.</li><li><strong>00:16 — Make something move.</strong> The First Light demo draws animated orbits and responds to color changes.</li><li><strong>00:24 — Make room for music.</strong> A moving wave accompanies the original score, exported by the ModernBASIC synth.</li><li><strong>00:32 — Linux, one key away.</strong> The installed OS’s Bash administration terminal opens with F12.</li><li><strong>00:40 — Your next idea starts at READY.</strong> Explore the manual. Try a first program. Make it yours.</li></ol></details>
</section>
<section><p class="eyebrow">FROM YOUR FIRST LINE TO YOUR NEXT IDEA</p><h2>A workspace made for making.</h2><div class="feature-grid"><article><span class="feature-number">01 / PROGRAM</span><h3>Start with a few lines.</h3><p>Numbered BASIC with modern values, structured procedures and a keyboard editor. Explore examples, change them and see what happens.</p><a href="/manual/guide-your-first-program">Write your first program →</a></article><article><span class="feature-number">02 / CREATE</span><h3>Give your ideas a screen and a voice.</h3><p>Draw graphics, animate sprites and make music. Each program has its own window and working state.</p><a href="/manual/language-graphics">Explore graphics →</a></article><article><span class="feature-number">03 / EXPLORE</span><h3>Linux is right underneath.</h3><p>Networking, files, printing and package tools are available through the administration terminal when you need them.</p><a href="/about">Meet the distribution →</a></article></div></section>
<section class="learning-band"><div><p class="eyebrow">THE MANUAL BELONGS HERE, TOO</p><h2>Learn it. Look it up. Keep going.</h2><p>Read the OS guide, browse BASIC statements, or find the audio command you need. The language reference comes from the same documentation as the built-in help.</p></div><a class="button" href="/manual">Open the user manual →</a></section>
<section id="downloads"><p class="eyebrow">CHOOSE YOUR HARDWARE</p><h2>Find your platform.</h2><p>Choose an image for your computer and check its compatibility notes before installation.</p><div class="grid">{cards}</div><p class="section-link"><a href="/downloads">Compare platforms and installation requirements →</a></p></section>
<section class="report-callout"><div><p class="eyebrow">BUILD A BETTER MODERNBASIC</p><h2>Help, updates and a way to be heard.</h2><p>Read the release notes, find a troubleshooting step, or send developers a report you have reviewed.</p></div><a class="button" href="/support">Visit support →</a></section>'''
    if path == '/about':
        return 'About the distribution', '<section>' + intro('MEET MODERNBASIC', 'An operating system that invites you to program.', 'A focused BASIC workspace with the Linux foundation you need to run a real computer.') + '''<div class="feature-grid"><article><h2>The BASIC experience</h2><p>Boot into the system hub, open multiple program windows and work at familiar READY prompts. The runtime supports checked 64-bit integers, real and complex values, strings, arrays, structured procedures, files, graphics and sound.</p></article><article><h2>The Linux foundation</h2><p>Debian supplies the kernel, drivers, networking and system services. ModernBASIC supplies the programming workspace and distro integration. The Linux administration terminal provides access to ordinary system tools.</p></article><article><h2>A place to learn</h2><p>Read the manual without leaving the OS. Browse a catalog of examples and games, then change the source. You can begin with PRINT and loops and grow into graphics, music and larger programs.</p></article></div><div class="prose"><h2>Who is it for?</h2><p>People who enjoy programming directly on their computer: learners, BASIC enthusiasts and makers exploring visual or musical ideas. It uses a focused workspace rather than a conventional application-launcher desktop.</p><h2>What to expect today</h2><p>The project is under active development. Development images have passed disposable-VM checks; public downloads and physical-hardware acceptance are still being prepared. Physical hardware coverage, firmware compatibility and demanding audio workloads require their own validation; the downloads page shows only images the release operator has accepted.</p><p>BASIC programs run locally. Some media and network features need supporting Linux tools or explicit host permission. The trusted-console administration terminal has full machine access.</p><p><a href="/start">Getting started</a> · <a href="/releases">Release status</a> · <a href="/licenses">Licenses and attribution</a></p></div></section>'''
    if path == '/downloads':
        fields = parse_qs(query, max_num_fields=4)
        selected = fields.get('platform', [''])[0]
        if selected and selected not in PLATFORM_DETAILS:
            raise ValueError('Unknown platform')
        choices = '<option value="">Compare all platforms</option>' + ''.join(f'<option value="{k}"{" selected" if k == selected else ""}>{e(v[0])}</option>' for k, v in PLATFORM_DETAILS.items())
        platforms = [selected] if selected else PLATFORM_DETAILS
        cards = ''.join(card(p, portal.downloads) for p in platforms)
        notes = ''.join(f'<article class="compatibility"><h3>{e(PLATFORM_DETAILS[p][0])}</h3><p>{e(PLATFORM_DETAILS[p][2])}</p></article>' for p in platforms)
        return 'Download ModernBASIC', '<section>' + intro('GET MODERNBASIC', 'The right image for your computer.', 'Choose your platform, read the compatibility notes, then verify your download before writing installation media.') + f'''<form class="platform-picker" method="get"><label for="platform">Your hardware</label><div><select id="platform" name="platform">{choices}</select><button>Show downloads</button></div></form><div class="grid downloads-grid">{cards}</div><div class="prose"><h2>Check compatibility first</h2>{notes}<p>Hardware minimums have not yet been validated for the forthcoming images. Check the release’s tested configurations; an architecture label is not a promise that every device using that processor will boot. A USB drive must be larger than the actual image file, and the installation disk needs space for the OS and your work.</p><h2>Verify your download</h2><p>Use the SHA-256 link next to the image and compare it with the hash calculated on your computer. Both the filename and the complete digest must match.</p><div class="tabs-static"><article><h3>Linux</h3><pre><code>sha256sum your-downloaded-image.iso</code></pre></article><article><h3>macOS</h3><pre><code>shasum -a 256 your-downloaded-image.iso</code></pre></article><article><h3>Windows PowerShell</h3><pre><code>Get-FileHash .\\your-downloaded-image.iso -Algorithm SHA256</code></pre></article></div><p>Replace the example filename with the image you downloaded, including its extension. Checksums detect corruption; use the official HTTPS download site to obtain both the image and its checksum.</p><p><a class="button" href="/start#installation">Continue to installation →</a></p></div></section>'''
    if path == '/start':
        return 'Getting started', '<section>' + intro('YOUR FIRST SESSION', 'From boot screen to first program.', 'Choose compatible hardware, try a live session, then settle into your own BASIC workspace.') + '''<div class="prose"><h2 id="installation">Install or try ModernBASIC</h2><ol class="steps"><li><h3>Choose and verify your image</h3><p>Use the <a href="/downloads">platform downloads</a> and read the matching release notes. Back up files before writing a USB drive or installing to a disk.</p></li><li><h3>Prepare the boot media</h3><p>Use your preferred image-writing utility to write the downloaded ISO as a disk image to a USB drive; copying the ISO as a normal file is not sufficient. Select the intended removable drive carefully, since image writing replaces its contents. Follow board-specific instructions for ARM, RISC-V or Pi images.</p></li><li><h3>Start a live session</h3><p>Boot the computer from that media and choose <strong>Start modernBASIC — live session</strong>. Test the display, keyboard, storage, networking and sound on your hardware. Live-session saves are temporary unless copied to persistent storage.</p></li><li><h3>Install when ready</h3><p>Choose <strong>Install modernBASIC</strong> from the boot menu. The Calamares installer guides language, keyboard, disk layout and administrator-account choices. Review its final summary before confirming disk changes. Installation copies the live system locally and does not require a network connection.</p></li><li><h3>Reboot into your workspace</h3><p>Remove the installation media when appropriate and start the installed system. Saved programs now live on its disk. Keep an independent backup of your work.</p></li></ol><h2>Create your first program</h2><p>At the system hub, enter:</p><pre><code>NEW "hello.bas"</code></pre><p>In the program window, enter these lines, then RUN:</p><pre><code>10 FOR I = 1 TO 5
20 PRINT "HELLO, WORLD!"
30 NEXT I
40 END
RUN
SAVE "hello.bas"</code></pre><p><a href="/manual/guide-your-first-program">Follow the full first-program guide →</a></p><h2>Five useful things to know</h2><ul><li>F1 opens help at the hub or a program’s READY prompt.</li><li>Ctrl+Escape interrupts a running BASIC program.</li><li>Use DIR to find examples and LOAD to open them.</li><li>COLORS lists the 32 names for your default screen and text colors.</li><li>SHUTDOWN at the hub asks for confirmation before powering off the appliance.</li></ul><p><a href="/manual">Read the user manual</a> · <a href="/support">Troubleshooting and support</a></p></div></section>'''
    if path == '/manual':
        groups = {}
        for doc in docs:
            if doc['group'] != 'Project':
                groups.setdefault(doc['group'], []).append(doc)
        sections = ''.join('<article class="manual-index"><h2>' + e(group) + '</h2><ul>' + ''.join(f'<li><a href="/manual/{d["slug"]}">{e(d["title"])}</a></li>' for d in entries) + '</ul></article>' for group, entries in groups.items())
        return 'User manual', '<section>' + intro('READ. TRY. CREATE.', 'The ModernBASIC user manual.', 'Everyday OS guidance, the BASIC language, graphics, audio and editing. Search for a command or explore a chapter.') + search_form() + '<p class="hint">The language, audio and editing chapters are generated from the project documentation. In the OS, press F1 for offline help.</p><div class="manual-index-grid">' + sections + '</div></section>'
    if path == '/manual/search':
        term = parse_qs(query, max_num_fields=4).get('q', [''])[0].strip()
        if len(term) > 100:
            raise ValueError('Search is too long')
        results = []
        if term:
            for doc in docs:
                if doc['group'] == 'Project':
                    continue
                index = doc['text'].casefold().find(term.casefold())
                if index >= 0 or term.casefold() in doc['title'].casefold():
                    start = max(0, index - 80)
                    snippet = ' '.join(doc['text'][start:start + 260].split())
                    results.append(f'<article class="search-result"><p class="eyebrow">{e(doc["group"])}</p><h2><a href="/manual/{doc["slug"]}">{e(doc["title"])}</a></h2><p>{e(snippet)}…</p></article>')
        body = ''.join(results) if results else '<p>No matching chapters. Try a command name, a shorter term, or <a href="/manual">browse the contents</a>.</p>'
        return 'Search the manual', '<section class="narrow wide">' + intro('USER MANUAL', 'Find the next step.', 'Search chapter titles and their full text.') + search_form(term) + f'<p>{len(results)} matching chapters</p>' + body + '</section>'
    if path.startswith('/manual/'):
        current = path.removeprefix('/manual/')
        manual = [d for d in docs if d['group'] != 'Project']
        for i, doc in enumerate(manual):
            if doc['slug'] == current:
                previous = f'<a href="/manual/{manual[i-1]["slug"]}">← {e(manual[i-1]["title"])}</a>' if i else '<a href="/manual">← Contents</a>'
                following = f'<a href="/manual/{manual[i+1]["slug"]}">{e(manual[i+1]["title"])} →</a>' if i + 1 < len(manual) else '<a href="/support">Support →</a>'
                return doc['title'], f'<section class="book-layout"><aside class="book-nav"><details><summary>Manual contents</summary>{manual_nav(docs, current)}</details><a href="/manual/search">Search the manual →</a></aside><article class="book-page"><p class="eyebrow">{e(doc["group"])}</p><h1>{e(doc["title"])}</h1><div class="prose">{doc["html"]}</div><nav class="chapter-links" aria-label="Chapter navigation">{previous}{following}</nav></article></section>'
        return None
    if path == '/releases':
        current = ''
        for key in PLATFORM_DETAILS:
            item = portal.downloads.get(key)
            if item:
                current += f'<article id="{key}" class="release-entry"><h2>{e(PLATFORM_DETAILS[key][0])} · {e(item["version"])}</h2><p>{e(item.get("notes", "Read the platform compatibility notes before installation."))}</p><p><a href="/downloads?platform={key}">Download and verify this image →</a></p><code class="checksum">SHA-256: {e(item["sha256"])}</code></article>'
        changes = next(d for d in docs if d['slug'] == 'changes')
        return 'Release notes and status', '<section>' + intro('FOLLOW THE PROJECT', 'Release notes & status.', 'See which images are available and what is changing in the programming environment.') + (current or '<div class="notice"><h2>Corrected images are in preparation.</h2><p>No accepted replacement images are published here yet. The previous revision-12 images predate the recent corrections. Each new image needs its own boot, installation and platform acceptance.</p></div>') + '<div class="prose"><h2>Known limitations</h2><ul><li>Generic ARM64 and RISC-V images require compatible firmware and board support.</li><li>A dedicated Pi 5 boot image has not been published.</li><li>Physical audio/display coverage and the reported demanding live-audio workload remain validation work.</li><li>Project-level licensing needs to be finalized before public redistribution; see <a href="/licenses">license status</a>.</li></ul><details class="release-history"><summary>Read the full development changelog</summary>' + changes['html'] + '</details></div></section>'
    if path == '/support':
        return 'Support and troubleshooting', '<section>' + intro('A LITTLE HELP GOES A LONG WAY', 'Find your way forward.', 'Start with the manual, check a common issue, or tell developers what happened.') + '''<div class="feature-grid"><article><h2>Learn and look up</h2><p>Find commands, key bindings and practical guidance in the user manual.</p><a href="/manual">Open the manual →</a></article><article><h2>Report a problem</h2><p>Separate queues cover Linux kernel, Modern BASIC and OS integration issues. Choose Unsure if the cause is unknown.</p><a href="/report">Send a bug report →</a></article><article><h2>See what changed</h2><p>Check available images, recent fixes and known limitations before upgrading.</p><a href="/releases">Read release notes →</a></article></div><div class="prose"><h2>Common questions</h2>
<details class="faq"><summary>A program is stuck. How do I stop it?</summary><p>Press Ctrl+Escape in its window. This requests BREAK and retains the working state. Use the hub’s TASKS and CLOSE commands if you need to end the task.</p></details>
<details class="faq"><summary>Where are my saved programs?</summary><p>Use PWD and DIR at READY. Installed appliance programs live under /home/modernbasic/programs; generated output is under /home/modernbasic/output. Live-session changes are temporary unless copied to persistent storage.</p></details>
<details class="faq"><summary>Why is there no sound or network connection?</summary><p>Open Settings for audio, display, Wi-Fi, Bluetooth, keyboard, time zone, and removable drives. For Linux tools, press F12 and use backend audio or backend wifi; nmtui is also available. BASIC network permission is separate from the OS connection. Describe your hardware and the smallest failing program when reporting a problem.</p></details>
<details class="faq"><summary>Does the OS automatically send diagnostics?</summary><p>No. Internal failure reports are stored locally. Once reporting is configured, a preview asks whether you want to send the report to the configured website. Only an explicit consent checkbox and Send report action upload it. Closing or canceling keeps it local. If the computer cannot display the prompt, successfully saved reports wait until the desktop is available.</p></details>
<details class="faq"><summary>What if the upload fails?</summary><p>Your local report is retained and no unattended retry is scheduled. Retry with diagnostics send REPORT in BASH; it asks for permission again. A domain must be configured before uploads can work.</p></details>
<details class="faq"><summary>Is this a Linux kernel bug or a BASIC bug?</summary><p>A language statement, editor or BASIC graphics/audio API problem usually belongs in Modern BASIC. A kernel panic or suspected driver issue belongs in Linux kernel. Installer, package and service wiring belongs in OS integration. Developers can reclassify your report.</p></details>
<h2>Contribute a useful report</h2><p>Include the build, platform, expected result, actual result and reproducible steps. A small BASIC example helps. Review logs and remove private details before sharing. Reports and developer notes are private; keep the reference number from the receipt.</p><p><a href="/privacy">Report privacy</a> · <a href="/developer">Developer workspace</a> · <a href="/licenses">Project and third-party notices</a></p><p>There is no public forum, mailing list or support email configured for this site yet. The report form is the available project contact route.</p></div></section>'''
    if path == '/licenses':
        license_doc = next(d for d in docs if d['slug'] == 'license')
        return 'Licenses and attribution', '<section class="narrow wide">' + intro('PROJECT NOTICES', 'Licenses & attribution.', 'ModernBASIC combines project code and assets with Linux and other third-party components.') + '<div class="prose">' + license_doc['html'] + '<p>Debian packages retain their copyright and license notices under /usr/share/doc in an installed system. A license applying to one component does not grant rights to unrelated project assets.</p></div></section>'
    return None
