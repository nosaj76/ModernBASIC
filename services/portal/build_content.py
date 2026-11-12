#!/usr/bin/env python3
"""Build or verify the portal's deployable manual bundle from checked-in docs."""
import argparse
import hashlib
import json
from pathlib import Path
from documents import chapters, render, slug

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = Path(__file__).with_name('content.json')
SOURCES = [('guide', 'Using the OS', 'services/portal/content/user-guide.md'),
           ('programming', 'Programming guide', 'docs/PROGRAMMING_GUIDE.md'),
           ('language', 'BASIC language', 'docs/LANGUAGE_REFERENCE.md'),
           ('trigonometry', 'Trigonometry', 'docs/TRIGONOMETRY.md'),
           ('native-math', 'Native math APIs', 'docs/NATIVE_MATH_API.md'),
           ('calendar', 'Calendar math', 'docs/CALENDAR_MATH.md'),
           ('schedules', 'Date schedules', 'docs/DATE_SCHEDULES.md'),
           ('discount-curves', 'Discount curves', 'docs/DISCOUNT_CURVES.md'),
           ('dated-irr', 'Dated returns', 'docs/DATED_IRR.md'),
           ('curve-bootstrap', 'Curve calibration', 'docs/CURVE_BOOTSTRAP.md'),
           ('multivariate-differentiation', 'Multivariable calculus', 'docs/MULTIVARIATE_DIFFERENTIATION.md'),
           ('hessian', 'Numerical Hessians', 'docs/NUMERICAL_HESSIANS.md'),
           ('physics-jacobians', 'Physics Jacobians', 'docs/PHYSICS_JACOBIANS.md'),
           ('audio', 'Audio reference', 'docs/AUDIO_REFERENCE.md'),
           ('editor', 'Files and editing', 'docs/EDITOR_AND_FILES.md')]


def build():
    documents, fingerprints = [], {}
    for prefix, group, relative in SOURCES:
        source = (ROOT / relative).read_text()
        fingerprints[relative] = hashlib.sha256(source.encode()).hexdigest()
        for title, body in chapters(source):
            documents.append(dict(slug=prefix + '-' + slug(title), title=title, group=group,
                                  source=relative, markdown='## ' + title + '\n\n' + body))
    links = {Path(relative).name: '/manual/' + next(d['slug'] for d in documents if d['source'] == relative)
             for _, _, relative in SOURCES}
    # Level-two Markdown headings become separate manual pages. Resolve only
    # those known chapter fragments; unknown targets remain unlinked.
    for document in documents:
        target = Path(document['source']).name + '#' + slug(document['title'])
        links.setdefault(target, '/manual/' + document['slug'])
    for document in documents:
        document['html'] = render(document['markdown'].split('\n', 1)[1], links)
        document['text'] = document.pop('markdown')
    for key, relative, title in [('changes', 'CHANGELOG.md', 'Development release notes'),
                                  ('license', 'LICENSE_STATUS.md', 'Licenses and attribution')]:
        source = (ROOT / relative).read_text()
        fingerprints[relative] = hashlib.sha256(source.encode()).hexdigest()
        documents.append(dict(slug=key, title=title, group='Project', source=relative,
                              html=render(source, links), text=source))
    return {'schema': 1, 'sources': fingerprints, 'documents': documents}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    encoded = json.dumps(build(), ensure_ascii=False, indent=2) + '\n'
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != encoded:
            raise SystemExit('Portal documentation is stale; run python3 services/portal/build_content.py')
        print('Portal documentation matches its source files.')
    else:
        OUTPUT.write_text(encoded)
        print('Built ' + str(OUTPUT))


if __name__ == '__main__':
    main()
