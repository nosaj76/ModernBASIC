"""Safe, dependency-free rendering of the project's documentation subset.

Raw HTML is always escaped. Documentation links resolve only to known pages or
HTTP(S) URLs. The generated bundle is read-only application content, not uploads.
"""
from html import escape
import re
from urllib.parse import urlsplit


def slug(text):
    return re.sub(r'[^a-z0-9]+', '-', text.lower()).strip('-') or 'section'


def inline(text, links):
    pattern = re.compile(r'`([^`]+)`|\[([^\]]+)\]\(([^)]+)\)|\*\*([^*]+)\*\*')
    out, position = [], 0
    for match in pattern.finditer(text):
        out.append(escape(text[position:match.start()]))
        code, label, target, bold = match.groups()
        if code is not None:
            out.append('<code>' + escape(code) + '</code>')
        elif bold is not None:
            out.append('<strong>' + escape(bold) + '</strong>')
        else:
            mapped = links.get(target)
            if mapped is None and urlsplit(target).scheme in ('https', 'http'):
                mapped = target
            out.append('<a href="' + escape(mapped, quote=True) + '">' + escape(label) + '</a>'
                       if mapped else escape(label))
        position = match.end()
    out.append(escape(text[position:]))
    return ''.join(out)


def render(markdown, links=None):
    links = links or {}
    lines = markdown.splitlines()
    output, paragraph, listing, code = [], [], None, None
    heading_counts = {}

    def flush():
        if paragraph:
            output.append('<p>' + inline(' '.join(paragraph), links) + '</p>')
            paragraph.clear()

    def close_list():
        nonlocal listing
        if listing:
            output.append('</li></' + listing + '>')
            listing = None

    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        if code is not None:
            if stripped.startswith('```'):
                output.append('<pre><code>' + escape('\n'.join(code)) + '</code></pre>')
                code = None
            else:
                code.append(line)
            i += 1
            continue
        if stripped.startswith('```'):
            flush(); close_list(); code = []; i += 1; continue
        heading = re.match(r'^(#{1,6})\s+(.+)$', line)
        if heading:
            flush(); close_list()
            level, text = len(heading[1]), heading[2]
            anchor = slug(text)
            heading_counts[anchor] = heading_counts.get(anchor, 0) + 1
            if heading_counts[anchor] > 1:
                anchor += '-' + str(heading_counts[anchor])
            output.append(f'<h{level} id="{anchor}">{inline(text, links)}</h{level}>')
        elif (stripped.startswith('|') and i + 1 < len(lines)
              and re.fullmatch(r'[\s|:\-]+', lines[i + 1]) and '-' in lines[i + 1]):
            flush(); close_list()
            def row(value, tag):
                return '<tr>' + ''.join(f'<{tag}>{inline(cell.strip(), links)}</{tag}>'
                                       for cell in value.strip().strip('|').split('|')) + '</tr>'
            output.append('<div class="table-scroll"><table><thead>' + row(line, 'th') + '</thead><tbody>')
            i += 2
            while i < len(lines) and lines[i].strip().startswith('|'):
                output.append(row(lines[i], 'td')); i += 1
            output.append('</tbody></table></div>')
            continue
        elif re.match(r'^\s*(?:[-*]|\d+\.)\s+', line):
            flush()
            item = re.match(r'^\s*([-*]|\d+\.)\s+(.+)', line)
            kind = 'ol' if item[1][0].isdigit() else 'ul'
            if listing != kind:
                close_list(); output.append('<' + kind + '>'); listing = kind
            else:
                output.append('</li>')
            output.append('<li>' + inline(item[2], links))
        elif not stripped:
            flush(); close_list()
        elif listing and line[:1].isspace():
            output.append(' ' + inline(stripped, links))
        else:
            close_list(); paragraph.append(stripped)
        i += 1
    flush(); close_list()
    if code is not None:
        output.append('<pre><code>' + escape('\n'.join(code)) + '</code></pre>')
    return '\n'.join(output)


def chapters(text):
    """Split level-two chapters without interpreting headings inside code."""
    result, title, body, fenced = [], None, [], False
    for line in text.splitlines():
        if line.strip().startswith('```'):
            fenced = not fenced
        if not fenced and line.startswith('## '):
            if title is not None:
                result.append((title, '\n'.join(body).strip()))
            elif any(line.strip() and not line.startswith('# ') for line in body):
                result.append(('Overview', '\n'.join(line for line in body if not line.startswith('# ')).strip()))
            title, body = line[3:].strip(), []
        else:
            body.append(line)
    if title is not None:
        result.append((title, '\n'.join(body).strip()))
    return result
