// The Script Editor's completion: the index of commands, procedures and Python names, the context at the caret, and
// the popup that offers what continues it. See docs/SCRIPT_EDITOR_COMPLETION.md for the behaviour.

#include "completion.h"

#include "host.h"

#include <QApplication>
#include <QFontMetrics>
#include <QJsonArray>
#include <QKeyEvent>
#include <QMouseEvent>
#include <QPainter>
#include <QPlainTextEdit>
#include <QRegularExpression>
#include <QSet>
#include <QTextBlock>
#include <QTextCursor>
#include <QWheelEvent>
#include <algorithm>

namespace az {

namespace {

constexpr int kRowHeight = 18;
constexpr int kMaxRows = 12;
constexpr int kDetailHeight = 34;
constexpr int kHintHeight = 22;

QFont mono_font(qreal size = 9.0) {
    QFont f(QStringLiteral("DejaVu Sans Mono"));
    f.setStyleHint(QFont::Monospace);
    f.setPointSizeF(size);
    return f;
}

QColor colour_for(CompletionEntry::Kind k) {
    switch (k) {
    case CompletionEntry::Module: return QColor(0x9f, 0xd0, 0xff);
    case CompletionEntry::Class: return QColor(0xd2, 0xa8, 0xff);
    case CompletionEntry::Function: return QColor(0xb5, 0xe0, 0x8a);
    case CompletionEntry::Method: return QColor(0x7f, 0xd4, 0xc9);
    case CompletionEntry::Flag: return QColor(0xc8, 0xd4, 0x8a);
    case CompletionEntry::Property: return QColor(0xe0, 0xc0, 0x80);
    case CompletionEntry::Constant: return QColor(0xb8, 0xb8, 0xb8);
    case CompletionEntry::Command: return QColor(0xb5, 0xe0, 0x8a);
    case CompletionEntry::Procedure: return QColor(0xe0, 0xb0, 0x80);
    }
    return QColor(0xd2, 0xd2, 0xd2);
}

QString glyph_for(CompletionEntry::Kind k) {
    switch (k) {
    case CompletionEntry::Module: return QStringLiteral("M");
    case CompletionEntry::Class: return QStringLiteral("C");
    case CompletionEntry::Function: return QStringLiteral("f");
    case CompletionEntry::Method: return QStringLiteral("m");
    case CompletionEntry::Flag: return QStringLiteral("-");
    case CompletionEntry::Property: return QStringLiteral("p");
    case CompletionEntry::Constant: return QStringLiteral("=");
    case CompletionEntry::Command: return QStringLiteral(">");
    case CompletionEntry::Procedure: return QStringLiteral(">");
    }
    return QStringLiteral("?");
}

QString initials(const QString& name) {
    QString out;
    for (int i = 0; i < name.size(); ++i) {
        const QChar c = name[i];
        if (i == 0) {
            out += c.toLower();
        } else if (c == '_') {
            continue;
        } else if (c.isUpper()) {
            const QChar prev = name[i - 1];
            if (prev.isLower() || prev.isDigit() || prev == '_') out += c.toLower();
            else if (i + 1 < name.size() && name[i + 1].isLower()) out += c.toLower();
        }
    }
    return out;
}

// How well `name` matches what is typed: lower is better, -1 is no match.
int rank_of(const QString& name, const QString& typed) {
    if (typed.isEmpty()) return 0;
    if (name.startsWith(typed, Qt::CaseSensitive)) return 0;
    if (name.startsWith(typed, Qt::CaseInsensitive)) return 1;
    if (initials(name).startsWith(typed, Qt::CaseInsensitive)) return 2;
    if (name.contains(typed, Qt::CaseInsensitive)) return 3;
    return -1;
}

bool in_comment_or_string(const QString& line, bool python) {
    QChar quote;
    bool escape = false;
    for (int i = 0; i < line.size(); ++i) {
        const QChar c = line[i];
        if (!quote.isNull()) {
            if (escape) {
                escape = false;
            } else if (c == '\\') {
                escape = true;
            } else if (c == quote) {
                quote = QChar();
            }
            continue;
        }
        if (c == '"' || c == '\'') {
            quote = c;
            continue;
        }
        if (python && c == '#') return true;
        if (!python && c == '/' && i + 1 < line.size() && line[i + 1] == '/') return true;
    }
    return !quote.isNull();
}

int last_unclosed_paren(const QString& line, int end, bool python) {
    QVector<int> opens;
    QChar quote;
    bool escape = false;
    for (int i = 0; i < end; ++i) {
        const QChar c = line[i];
        if (!quote.isNull()) {
            if (escape) {
                escape = false;
            } else if (c == '\\') {
                escape = true;
            } else if (c == quote) {
                quote = QChar();
            }
            continue;
        }
        if (c == '"' || c == '\'') {
            quote = c;
            continue;
        }
        if (python && c == '#') break;
        if (!python && c == '/' && i + 1 < end && line[i + 1] == '/') break;
        if (c == '(') {
            opens.append(i);
        } else if (c == ')' && !opens.isEmpty()) {
            opens.removeLast();
        }
    }
    return opens.isEmpty() ? -1 : opens.last();
}

struct Inside {
    int argument = 0;
    int start = 0;
    QString prefix;
};

// The argument under the caret: its index, where it starts, and the identifier typed in it.
Inside inside_call(const QString& line, int open, int end, bool python) {
    Inside in;
    in.argument = 0;
    in.start = open + 1;
    int depth = 0;
    QChar quote;
    bool escape = false;
    for (int i = open + 1; i < end; ++i) {
        const QChar c = line[i];
        if (!quote.isNull()) {
            if (escape) {
                escape = false;
            } else if (c == '\\') {
                escape = true;
            } else if (c == quote) {
                quote = QChar();
            }
            continue;
        }
        if (c == '"' || c == '\'') {
            quote = c;
            continue;
        }
        if (python && c == '#') break;
        if (!python && c == '/' && i + 1 < end && line[i + 1] == '/') break;
        if (c == '(' || c == '[' || c == '{') {
            ++depth;
        } else if (c == ')' || c == ']' || c == '}') {
            --depth;
        } else if (c == ',' && depth == 0) {
            ++in.argument;
            in.start = i + 1;
        }
    }
    while (in.start < end && line[in.start].isSpace()) ++in.start;
    const QString typed = line.mid(in.start, end - in.start);
    static const QRegularExpression ident("^[A-Za-z_][A-Za-z0-9_]*$");
    if (ident.match(typed).hasMatch()) in.prefix = typed;
    return in;
}

QStringList split_parameters(const QString& text) {
    QStringList out;
    for (QString part : text.split(',', Qt::SkipEmptyParts)) {
        part = part.trimmed();
        if (part.startsWith('[') && part.endsWith(']')) part = part.mid(1, part.size() - 2);
        if (!part.isEmpty()) out << part;
    }
    return out;
}

QString python_doc(const QJsonObject& o) {
    const QString d = o.value("doc").toString().trimmed();
    return d;
}

} // namespace

ScriptCompletion::ScriptCompletion(QWidget* host) : QWidget(host), host_(host) {
    setObjectName("scriptCompletion");
    setFocusPolicy(Qt::NoFocus);
    setMouseTracking(true);
    setAttribute(Qt::WA_ShowWithoutActivating);
    hide();
    host_->installEventFilter(this);
}

ScriptCompletion::~ScriptCompletion() = default;

void ScriptCompletion::set_enabled(bool on) {
    enabled_ = on;
    if (!on) dismiss();
}

void ScriptCompletion::dismiss() {
    editor_.clear();
    base_.clear();
    entries_.clear();
    hint_ = false;
    hint_name_.clear();
    hint_params_.clear();
    hint_current_ = -1;
    replace_start_ = -1;
    replace_end_ = -1;
    applied_key_.clear();
    applied_prefix_.clear();
    selected_ = 0;
    scroll_ = 0;
    hide();
}

void ScriptCompletion::ensure_index() {
    if (loaded_) return;
    loaded_ = true;
    const QJsonObject v = query("completionCore").toObject();
    for (const QJsonValue& cv : v.value("commands").toArray()) {
        const QJsonObject o = cv.toObject();
        Command c;
        c.name = o.value("name").toString();
        c.doc = o.value("doc").toString();
        c.positional = o.value("positional").toString();
        for (const QJsonValue& fv : o.value("flags").toArray()) {
            const QJsonObject f = fv.toObject();
            Flag flag;
            flag.name = f.value("name").toString();
            flag.shortName = f.value("short").toString();
            flag.type = f.value("type").toString();
            flag.doc = f.value("doc").toString();
            c.flags.append(flag);
        }
        commands_.insert(c.name, c);
    }
    for (const QJsonValue& pv : v.value("procedures").toArray()) {
        const QJsonObject o = pv.toObject();
        Proc p;
        p.name = o.value("name").toString();
        p.signature = o.value("signature").toString();
        p.doc = o.value("doc").toString();
        procs_.insert(p.name, p);
    }
    for (const QJsonValue& bv : v.value("builtins").toArray()) {
        const QJsonObject o = bv.toObject();
        builtins_ << o.value("name").toString();
    }
    builtins_.sort(Qt::CaseInsensitive);
    for (const QJsonValue& mv : v.value("melOnly").toArray()) mel_only_.insert(mv.toString());
}

void ScriptCompletion::load_python(const QJsonObject& py) {
    const QJsonObject modules = py.value("modules").toObject();
    for (auto it = modules.constBegin(); it != modules.constEnd(); ++it) {
        const QJsonObject o = it.value().toObject();
        PyModule m;
        m.name = it.key();
        m.doc = o.value("doc").toString();
        for (const QJsonValue& ev : o.value("entries").toArray()) {
            const QJsonObject e = ev.toObject();
            PyEntry entry;
            entry.name = e.value("name").toString();
            entry.kind = e.value("kind").toString();
            entry.signature = e.value("signature").toString();
            entry.doc = python_doc(e);
            m.entries.append(entry);
        }
        for (const QJsonValue& cv : o.value("children").toArray()) m.children << cv.toString();
        py_modules_.insert(m.name, m);
    }
}

// The procedures the session defines while the panel is open: the index's list was read once, so they are asked for
// again when the command list is prepared or a call's hint is wanted.
void ScriptCompletion::refresh_live_procedures() {
    for (const QJsonValue& pv : query("liveProcedures").toArray()) {
        const QJsonObject o = pv.toObject();
        const QString name = o.value("name").toString();
        if (name.isEmpty()) continue;
        Proc p;
        p.name = name;
        p.signature = o.value("signature").toString();
        p.doc = o.value("doc").toString();
        procs_.insert(name, p);
    }
}

void ScriptCompletion::ensure_python() {
    if (python_loaded_) return;
    python_loaded_ = true;
    const QJsonObject v = completion_index();
    py_error_ = v.value("pythonError").toString();
    load_python(v.value("python").toObject());
}

const ScriptCompletion::PyClass* ScriptCompletion::py_class(const QString& module, const QString& name) {
    const QString key = module + "." + name;
    auto it = py_classes_.find(key);
    if (it != py_classes_.end() && it->loaded) return &*it;
    const QJsonObject v = completion_class(module, name);
    PyClass c;
    c.name = name;
    c.module = module;
    c.loaded = true;
    if (v.contains("error")) {
        c.doc = v.value("error").toString();
    } else if (!v.isEmpty()) {
        c.doc = v.value("doc").toString();
        c.init = v.value("init").toString();
        for (const QJsonValue& mv : v.value("members").toArray()) {
            const QJsonObject m = mv.toObject();
            PyMember member;
            member.name = m.value("name").toString();
            member.kind = m.value("kind").toString();
            member.signature = m.value("signature").toString();
            member.doc = m.value("doc").toString();
            member.inherited = m.value("inherited").toBool();
            if (!member.name.isEmpty()) c.members.append(member);
        }
    }
    auto inserted = py_classes_.insert(key, c);
    return &*inserted;
}

QString ScriptCompletion::aliased(const QString& expression, QPlainTextEdit* editor) const {
    if (expression.isEmpty()) return expression;
    const QString first = expression.section('.', 0, 0);
    QString target;
    int scanned = 0;
    for (QTextBlock b = editor->document()->begin(); b.isValid() && scanned < 4000; b = b.next(), ++scanned) {
        const QString text = b.text().trimmed();
        if (!text.startsWith("import ") && !text.startsWith("from ")) continue;
        if (text.startsWith("import ")) {
            static const QRegularExpression as_import(R"(^import\s+([A-Za-z_][\w.]*)\s+as\s+([A-Za-z_]\w*)$)");
            const auto m = as_import.match(text);
            if (m.hasMatch() && m.captured(2) == first) target = m.captured(1);
        } else {
            static const QRegularExpression from_import(R"(^from\s+([A-Za-z_][\w.]*)\s+import\s+([A-Za-z_]\w*)(?:\s+as\s+([A-Za-z_]\w*))?)");
            const auto m = from_import.match(text);
            if (m.hasMatch()) {
                const QString name = m.captured(3).isEmpty() ? m.captured(2) : m.captured(3);
                if (name == first) target = m.captured(1) + "." + m.captured(2);
            }
        }
        if (!target.isEmpty()) return target + expression.mid(first.size());
    }
    return expression;
}

QString ScriptCompletion::python_signature(const Command& c) const {
    return c.name + "(*args, **kwargs)";
}

ScriptCompletion::Analysis ScriptCompletion::analyse(QPlainTextEdit* editor) {
    Analysis a;
    if (!editor) return a;
    const QTextCursor cursor = editor->textCursor();
    const QTextBlock block = cursor.block();
    const int col = cursor.positionInBlock();
    const QString full = block.text();
    const QString line = full.left(col);
    a.python = editor->property("scriptLanguage").toString() == "python";
    if (in_comment_or_string(line, a.python)) return a;

    if (a.python) {
        static const QRegularExpression ident(R"(([A-Za-z_][A-Za-z0-9_]*)$)");
        QString prefix;
        int word_start = line.size();
        const auto im = ident.match(line);
        if (im.hasMatch() && im.capturedEnd() == line.size()) {
            prefix = im.captured(1);
            word_start = im.capturedStart(1);
        }
        const bool after_dot = word_start > 0 && line[word_start - 1] == '.';
        if (after_dot) {
            static const QRegularExpression expr(R"(([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)$)");
            const auto em = expr.match(line.left(word_start - 1));
            if (em.hasMatch()) {
                const QString base = aliased(em.captured(1), editor);
                a.mode = Analysis::Members;
                a.base = base;
                a.prefix = prefix;
                a.replace_start = block.position() + word_start;
                a.replace_end = cursor.position();
                return a;
            }
        }
        const int open = last_unclosed_paren(line, col, true);
        if (open >= 0) {
            static const QRegularExpression callee(R"(([A-Za-z_][A-Za-z0-9_.]*)\s*$)");
            const auto cm = callee.match(line.left(open));
            if (cm.hasMatch()) {
                const QString name = aliased(cm.captured(1), editor);
                const Inside in = inside_call(line, open, col, true);
                QString command_name = name;
                if (name.startsWith("maya.cmds.")) command_name = name.mid(QStringLiteral("maya.cmds.").size());
                else if (name.startsWith("aztek.cmds.")) command_name = name.mid(QStringLiteral("aztek.cmds.").size());
                else if (name.contains('.')) command_name.clear();
                const auto ci = commands_.constFind(command_name);
                if (ci != commands_.constEnd() && !command_name.isEmpty()) {
                    a.mode = Analysis::Hint;
                    a.command = &*ci;
                    a.prefix = in.prefix;
                    a.replace_start = block.position() + in.start;
                    a.replace_end = cursor.position();
                    a.hint_name = command_name;
                    const QString typed = line.mid(in.start, col - in.start).trimmed();
                    a.offer_entries = typed.isEmpty() || !in.prefix.isEmpty();
                    int flags = 0;
                    for (const Flag& f : ci->flags) {
                        if (flags++ >= 6) {
                            a.hint_params << QStringLiteral("...");
                            break;
                        }
                        a.hint_params << f.name + "=";
                    }
                    a.hint_current = in.argument;
                    return a;
                }
                int dot = name.lastIndexOf('.');
                if (dot > 0) {
                    const QString module = name.left(dot);
                    const QString cls = name.mid(dot + 1);
                    if (py_modules_.contains(module)) {
                        const PyClass* pc = py_class(module, cls);
                        if (pc && !pc->init.isEmpty()) {
                            a.mode = Analysis::Hint;
                            a.prefix = in.prefix;
                            a.replace_start = block.position() + in.start;
                            a.replace_end = cursor.position();
                            a.hint_name = cls;
                            a.hint_params = split_parameters(pc->init.mid(1, pc->init.size() - 2));
                            a.hint_current = in.argument;
                            return a;
                        }
                    }
                }
            }
        }
        if (!prefix.isEmpty()) {
            // Writing an import: the names in it are not completion, and Enter should end the line rather than
            // accept an alias that spells what is already there.
            const QString trimmed = line.trimmed();
            if (trimmed.startsWith(QLatin1String("import ")) || trimmed.startsWith(QLatin1String("from "))) return a;
            a.mode = Analysis::Words;
            a.prefix = prefix;
            a.replace_start = block.position() + word_start;
            a.replace_end = cursor.position();
            return a;
        }
        return a;
    }

    // MEL: the word ending at the caret decides. A command or procedure followed by an unclosed parenthesis
    // shows its argument hint; a flag shows the flags; a command name at the start of a statement, or after an
    // operator or a backtick, shows the commands and procedures.
    const int open = last_unclosed_paren(line, line.size(), false);
    if (open >= 0) {
        static const QRegularExpression callee(R"(([A-Za-z_][A-Za-z0-9_]*)\s*$)");
        const auto cm = callee.match(line.left(open));
        if (cm.hasMatch()) {
            const QString callee_name = cm.captured(1);
            const auto ci = commands_.constFind(callee_name);
            if (ci == commands_.constEnd() && !procs_.contains(callee_name)) refresh_live_procedures();
            if (ci != commands_.constEnd() || procs_.contains(callee_name)) {
                const Inside in = inside_call(line, open, line.size(), false);
                a.mode = Analysis::Hint;
                a.prefix = in.prefix;
                a.replace_start = block.position() + in.start;
                a.replace_end = cursor.position();
                a.hint_name = callee_name;
                if (ci != commands_.constEnd()) {
                    a.command = &*ci;
                    a.hint_params = split_parameters(ci->positional);
                    if (a.hint_params.isEmpty() && !ci->flags.isEmpty()) a.hint_params << QStringLiteral("flags");
                } else {
                    const QString signature = procs_.value(callee_name).signature;
                    const int p = signature.indexOf('(');
                    if (p >= 0) a.hint_params = split_parameters(signature.mid(p + 1, signature.lastIndexOf(')') - p - 1));
                }
                a.hint_current = in.argument;
                return a;
            }
        }
    }

    static const QRegularExpression trailing(R"(([A-Za-z_][A-Za-z0-9_]*)$)");
    const auto tm = trailing.match(line);
    const int word_start = tm.hasMatch() ? tm.capturedStart(1) : line.size();
    const QString word = tm.hasMatch() ? tm.captured(1) : QString();
    int before = word_start - 1;
    while (before >= 0 && line[before].isSpace()) --before;

    // A flag: the dash right before the word, and the command before the dash.
    if (before >= 0 && line[before] == '-') {
        int command_end = before - 1;
        while (command_end >= 0 && line[command_end].isSpace()) --command_end;
        int command_start = command_end;
        while (command_start >= 0 && (line[command_start].isLetterOrNumber() || line[command_start] == '_')) --command_start;
        const QString command = line.mid(command_start + 1, command_end - command_start);
        const auto ci = commands_.constFind(command);
        if (ci != commands_.constEnd()) {
            a.mode = Analysis::Flags;
            a.command = &*ci;
            a.prefix = word;
            a.replace_start = block.position() + before;
            a.replace_end = cursor.position();
            a.hint_name = command;
            return a;
        }
    }

    // The caret after a command and a space: its flags.
    if (word.isEmpty() && !line.isEmpty() && line.at(line.size() - 1).isSpace()) {
        int command_end = line.size() - 1;
        while (command_end >= 0 && line[command_end].isSpace()) --command_end;
        int command_start = command_end;
        while (command_start >= 0 && (line[command_start].isLetterOrNumber() || line[command_start] == '_')) --command_start;
        const QString command = line.mid(command_start + 1, command_end - command_start);
        const auto ci = commands_.constFind(command);
        if (ci != commands_.constEnd()) {
            int p = command_start;
            while (p >= 0 && line[p].isSpace()) --p;
            const bool statement_start = p < 0 || line.left(p + 1).section(';', -1).trimmed().isEmpty();
            const QChar prev = p >= 0 ? line[p] : QChar();
            if (statement_start || QStringLiteral("=([{,`").contains(prev)) {
                a.mode = Analysis::Flags;
                a.command = &*ci;
                a.prefix.clear();
                a.replace_start = cursor.position();
                a.replace_end = cursor.position();
                a.hint_name = command;
                return a;
            }
        }
    }

    // A command name being typed: the start of a statement, or after an operator or a backtick.
    if (!word.isEmpty()) {
        const bool statement_start = line.left(word_start).section(';', -1).trimmed().isEmpty();
        const QChar prev = before >= 0 ? line[before] : QChar();
        if (statement_start || QStringLiteral("=([{,`").contains(prev)) {
            a.mode = Analysis::Words;
            a.prefix = word;
            a.replace_start = block.position() + word_start;
            a.replace_end = cursor.position();
            return a;
        }
    }
    return a;
}

void ScriptCompletion::add_module_members(const QString& module) {
    if (module == "maya.cmds") {
        add_command_candidates(true);
        return;
    }
    const auto it = py_modules_.constFind(module);
    if (it == py_modules_.constEnd()) {
        CompletionEntry e;
        e.label = py_error_.isEmpty() ? QStringLiteral("Python API unavailable") : py_error_;
        e.insert.clear();
        e.kind = CompletionEntry::Module;
        e.category = QStringLiteral("not available");
        e.detail = e.label;
        e.enabled = false;
        base_.append(e);
        return;
    }
    const PyModule& m = *it;
    QSet<QString> added;
    for (const PyEntry& e : m.entries) {
        CompletionEntry ce;
        ce.label = e.name;
        ce.insert = e.name;
        ce.doc = e.doc;
        if (e.kind == "class") {
            ce.kind = CompletionEntry::Class;
            ce.category = QStringLiteral("class");
            ce.detail = e.name + e.signature;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Class in %1.").arg(module);
        } else if (e.kind == "module") {
            ce.kind = CompletionEntry::Module;
            ce.category = QStringLiteral("module");
            ce.detail = e.name;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Module in %1.").arg(module);
        } else if (e.kind == "function") {
            ce.kind = CompletionEntry::Function;
            ce.category = QStringLiteral("function");
            ce.detail = e.name + e.signature;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Function in %1.").arg(module);
        } else {
            ce.kind = CompletionEntry::Constant;
            ce.category = QStringLiteral("constant");
            ce.detail = e.name;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Constant in %1.").arg(module);
        }
        base_.append(ce);
        added.insert(e.name);
    }
    for (const QString& child : m.children) {
        if (added.contains(child)) continue;
        CompletionEntry ce;
        ce.label = child;
        ce.insert = child;
        ce.kind = CompletionEntry::Module;
        ce.category = QStringLiteral("module");
        ce.detail = module + "." + child;
        const auto mi = py_modules_.constFind(module + "." + child);
        if (mi == py_modules_.constEnd() || (mi->entries.isEmpty() && mi->children.isEmpty())) {
            ce.enabled = false;
            ce.doc = QStringLiteral("Empty package in Aztek.");
            ce.category = QStringLiteral("not available");
        } else {
            ce.doc = mi->doc;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Python module.");
        }
        base_.append(ce);
    }
}

void ScriptCompletion::add_class_members(const QString& module, const QString& name) {
    const PyClass* c = py_class(module, name);
    if (!c) return;
    for (const PyMember& m : c->members) {
        CompletionEntry ce;
        ce.label = m.name;
        ce.insert = m.name;
        ce.doc = m.doc;
        if (m.kind == "method") {
            ce.kind = CompletionEntry::Method;
            ce.category = QStringLiteral("method");
            ce.detail = m.name + m.signature;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Method of %1.").arg(name);
        } else if (m.kind == "property") {
            ce.kind = CompletionEntry::Property;
            ce.category = QStringLiteral("property");
            ce.detail = m.name;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Property of %1.").arg(name);
        } else if (m.kind == "class") {
            ce.kind = CompletionEntry::Class;
            ce.category = QStringLiteral("class");
            ce.detail = m.name;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Class in %1.").arg(module);
        } else if (m.kind == "function") {
            ce.kind = CompletionEntry::Function;
            ce.category = QStringLiteral("function");
            ce.detail = m.name + m.signature;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Function of %1.").arg(name);
        } else {
            ce.kind = CompletionEntry::Constant;
            ce.category = QStringLiteral("constant");
            ce.detail = m.name;
            if (ce.doc.isEmpty()) ce.doc = QStringLiteral("Constant of %1.").arg(name);
        }
        base_.append(ce);
    }
}

void ScriptCompletion::add_command_candidates(bool python) {
    for (const Command& c : commands_) {
        CompletionEntry e;
        e.label = c.name;
        e.insert = c.name;
        e.kind = CompletionEntry::Function;
        e.category = python ? QStringLiteral("function") : QStringLiteral("command");
        e.detail = python ? python_signature(c) : (c.positional.isEmpty() ? c.name : c.name + " <" + c.positional + ">");
        e.doc = c.doc;
        // Nine commands are MEL only: maya.cmds has no function for them. Shown greyed rather than hidden.
        if (python && mel_only_.contains(c.name)) {
            e.enabled = false;
            e.category = QStringLiteral("MEL only");
            e.doc = QStringLiteral("MEL only: maya.cmds has no function for this command.");
        }
        base_.append(e);
    }
}

void ScriptCompletion::add_flag_candidates(const Command& c, bool python) {
    for (const Flag& f : c.flags) {
        if (python) {
            CompletionEntry e;
            e.label = f.name + "=";
            e.insert = f.name + "=";
            e.kind = CompletionEntry::Flag;
            e.category = QStringLiteral("flag ") + f.type;
            e.detail = f.name + " <" + f.type + ">";
            e.doc = f.doc;
            base_.append(e);
            continue;
        }
        CompletionEntry long_form;
        long_form.label = "-" + f.name;
        long_form.insert = "-" + f.name + " ";
        long_form.kind = CompletionEntry::Flag;
        long_form.category = QStringLiteral("flag ") + f.type;
        long_form.detail = "-" + f.name + " <" + f.type + ">";
        if (!f.shortName.isEmpty()) long_form.detail += "  short: -" + f.shortName;
        long_form.doc = f.doc;
        base_.append(long_form);
        if (!f.shortName.isEmpty()) {
            CompletionEntry short_form;
            short_form.label = "-" + f.shortName;
            short_form.insert = "-" + f.shortName + " ";
            short_form.kind = CompletionEntry::Flag;
            short_form.category = QStringLiteral("flag ") + f.type;
            short_form.detail = "-" + f.shortName + " <" + f.type + ">";
            short_form.doc = QStringLiteral("Short form of -%1.").arg(f.name);
            if (!f.doc.isEmpty()) short_form.doc += " " + f.doc;
            base_.append(short_form);
        }
    }
}

void ScriptCompletion::build_candidates(const Analysis& a) {
    base_.clear();
    // Flags read in the registry's order, a long form and then its short one; names are ranked by fit.
    preserve_order_ = a.mode == Analysis::Flags || a.mode == Analysis::Hint;
    switch (a.mode) {
    case Analysis::Members:
        if (a.python && (a.base == "maya.cmds" || a.base == "aztek.cmds")) {
            add_command_candidates(true);
        } else if (a.python && py_modules_.contains(a.base)) {
            add_module_members(a.base);
        } else if (a.python && py_modules_.isEmpty()) {
            add_module_members(a.base);
        } else if (a.python) {
            const int dot = a.base.lastIndexOf('.');
            if (dot > 0) {
                const QString module = a.base.left(dot);
                const QString cls = a.base.mid(dot + 1);
                if (py_modules_.contains(module)) {
                    bool is_class = false;
                    for (const PyEntry& e : py_modules_.value(module).entries) {
                        if (e.kind == "class" && e.name == cls) {
                            is_class = true;
                            break;
                        }
                    }
                    if (is_class) add_class_members(module, cls);
                }
            }
        }
        break;
    case Analysis::Flags:
        if (a.command) add_flag_candidates(*a.command, a.python);
        break;
    case Analysis::Hint:
        if (a.command && a.python && a.offer_entries) add_flag_candidates(*a.command, true);
        break;
    case Analysis::Words:
        if (a.python) {
            QSet<QString> aliases;
            for (QTextBlock b = editor_ ? editor_->document()->begin() : QTextBlock(); b.isValid(); b = b.next()) {
                const QString text = b.text().trimmed();
                static const QRegularExpression as_import(R"(^import\s+[A-Za-z_][\w.]*\s+as\s+([A-Za-z_]\w*)$)");
                static const QRegularExpression from_import(R"(^from\s+[A-Za-z_][\w.]*\s+import\s+[A-Za-z_]\w*(?:\s+as\s+([A-Za-z_]\w*))?)");
                const auto am = as_import.match(text);
                if (am.hasMatch()) aliases.insert(am.captured(1));
                const auto fm = from_import.match(text);
                if (fm.hasMatch() && !fm.captured(1).isEmpty()) aliases.insert(fm.captured(1));
            }
            for (const QString& name : {"maya", "aztek"}) {
                CompletionEntry e;
                e.label = name;
                e.insert = name;
                e.kind = CompletionEntry::Module;
                e.category = QStringLiteral("module");
                e.detail = name;
                e.doc = name == "maya" ? QStringLiteral("Aztek's maya compatibility package.") : QStringLiteral("Aztek's own package.");
                base_.append(e);
            }
            for (const QString& alias : aliases) {
                CompletionEntry e;
                e.label = alias;
                e.insert = alias;
                e.kind = CompletionEntry::Module;
                e.category = QStringLiteral("module");
                e.detail = alias;
                e.doc = QStringLiteral("Imported module.");
                base_.append(e);
            }
        } else {
            QSet<QString> seen;
            for (const Command& c : commands_) {
                seen.insert(c.name);
                CompletionEntry e;
                e.label = c.name;
                e.insert = c.name;
                e.kind = CompletionEntry::Command;
                e.category = QStringLiteral("command");
                e.detail = c.positional.isEmpty() ? c.name : c.name + " <" + c.positional + ">";
                e.doc = c.doc;
                base_.append(e);
            }
            refresh_live_procedures();
            for (const Proc& p : procs_) {
                if (seen.contains(p.name)) continue;
                CompletionEntry e;
                e.label = p.name;
                e.insert = p.name;
                e.kind = CompletionEntry::Procedure;
                e.category = QStringLiteral("procedure");
                e.detail = p.signature;
                e.doc = p.doc;
                base_.append(e);
            }
            for (const QString& b : builtins_) {
                if (seen.contains(b)) continue;
                CompletionEntry e;
                e.label = b;
                e.insert = b;
                e.kind = CompletionEntry::Function;
                e.category = QStringLiteral("function");
                e.detail = b;
                e.doc = QStringLiteral("MEL built-in function.");
                base_.append(e);
            }
        }
        break;
    case Analysis::None:
        break;
    }
}

void ScriptCompletion::apply_filter() {
    QVector<QPair<int, CompletionEntry>> ranked;
    ranked.reserve(base_.size());
    for (const CompletionEntry& e : base_) {
        const int fit = rank_of(e.label, applied_prefix_);
        if (fit < 0) continue;
        // A greyed entry keeps its place by fit: after an enabled entry of the same fit, before a worse fit.
        ranked.append({fit * 2 + (e.enabled ? 0 : 1), e});
    }
    const bool unfiltered = applied_prefix_.isEmpty();
    std::stable_sort(ranked.begin(), ranked.end(), [this, unfiltered](const auto& a, const auto& b) {
        if (a.first != b.first) return a.first < b.first;
        if (preserve_order_) return false;
        if (unfiltered) return a.second.label.compare(b.second.label, Qt::CaseInsensitive) < 0;
        if (a.second.label.size() != b.second.label.size()) return a.second.label.size() < b.second.label.size();
        return a.second.label.compare(b.second.label, Qt::CaseInsensitive) < 0;
    });
    entries_.clear();
    entries_.reserve(ranked.size());
    for (const auto& r : ranked) entries_.append(r.second);
}

void ScriptCompletion::refresh(QPlainTextEdit* editor) {
    if (!enabled_ || !editor) {
        dismiss();
        return;
    }
    ensure_index();
    if (editor->property("scriptLanguage").toString() == "python") ensure_python();
    const Analysis a = analyse(editor);
    if (a.mode == Analysis::None) {
        dismiss();
        return;
    }
    editor_ = editor;
    replace_start_ = a.replace_start;
    replace_end_ = a.replace_end;
    hint_ = false;
    hint_name_ = a.hint_name;
    hint_params_ = a.hint_params;
    hint_current_ = a.hint_current;
    if (a.mode == Analysis::Hint) hint_ = true;

    QString key = QString::number(static_cast<int>(a.mode)) + "|" + a.base + "|" + (a.command ? a.command->name : QString()) + "|" + (a.python ? "py" : "mel");
    // Python word completion also carries the import aliases, so it follows an import added while typing.
    if (a.mode == Analysis::Words && a.python) key += "|" + QString::number(editor->document()->revision());
    if (key != applied_key_) {
        applied_key_ = key;
        applied_prefix_.clear();
        selected_ = 0;
        scroll_ = 0;
        build_candidates(a);
    }
    if (a.prefix != applied_prefix_) {
        applied_prefix_ = a.prefix;
        selected_ = 0;
        scroll_ = 0;
    }
    apply_filter();
    // The first argument of a call is not a keyword argument; do not hide a useful hint over an empty list.
    if (entries_.isEmpty() && !hint_) {
        dismiss();
        return;
    }
    if (hint_ && hint_current_ >= hint_params_.size()) hint_current_ = hint_params_.isEmpty() ? -1 : hint_params_.size() - 1;
    // Prefer the space under the caret; go above only when that is too small.
    const QRect caret = editor->cursorRect();
    const QPoint top = editor->viewport()->mapTo(host_, caret.topLeft());
    const int below = host_->height() - (top.y() + caret.height() + 4);
    const int above = top.y() - 4;
    place_above_ = below < 120 && above > below;
    layout_popup(qMax(40, place_above_ ? above : below));
    place();
    show();
    raise();
    update();
}

void ScriptCompletion::layout_popup(int max_height) {
    const QFontMetrics fm(mono_font(9.5));
    int width = 300;
    int measured = 0;
    for (const CompletionEntry& e : entries_) {
        if (measured++ > 400) break;
        const int row = 30 + fm.horizontalAdvance(e.label) + 16 + fm.horizontalAdvance(e.category);
        width = qMax(width, row);
    }
    if (hint_) {
        QString text = hint_name_ + "(" + hint_params_.join(", ") + ")";
        width = qMax(width, 20 + fm.horizontalAdvance(text));
    }
    width = qBound(280, width, 660);
    width = qMin(width, qMax(160, host_->width() - 6));
    visible_rows_ = qMin(kMaxRows, entries_.size());
    if (visible_rows_ < 1 && hint_) visible_rows_ = 0;
    const int detail = entries_.isEmpty() ? 0 : kDetailHeight;
    int height = 2 + hint_height() + visible_rows_ * kRowHeight + detail;
    const int available = qMax(40, max_height);
    while (height > available && visible_rows_ > 2) {
        --visible_rows_;
        height = 2 + hint_height() + visible_rows_ * kRowHeight + detail;
    }
    if (height > available) height = available;
    resize(width, height);
    if (selected_ < scroll_) scroll_ = selected_;
    if (selected_ >= scroll_ + visible_rows_) scroll_ = selected_ - visible_rows_ + 1;
    const int last = qMax(0, entries_.size() - visible_rows_);
    scroll_ = qBound(0, scroll_, last);
}

int ScriptCompletion::detail_height() const {
    return entries_.isEmpty() ? 0 : kDetailHeight;
}

void ScriptCompletion::place() {
    if (!editor_) return;
    const QRect caret = editor_->cursorRect();
    const QPoint top = editor_->viewport()->mapTo(host_, caret.topLeft());
    const QSize sz = size();
    QRect bounds = host_->rect().adjusted(2, 2, -2, -2);
    int x = top.x();
    int y = place_above_ ? top.y() - sz.height() - 2 : top.y() + caret.height() + 2;
    x = qBound(bounds.left(), x, qMax(bounds.left(), bounds.right() - sz.width()));
    y = qBound(bounds.top(), y, qMax(bounds.top(), bounds.bottom() - sz.height()));
    setGeometry(QRect(QPoint(x, y), sz));
}

bool ScriptCompletion::key_press(QKeyEvent* e, QPlainTextEdit* editor) {
    if (!enabled_ || !isVisible() || editor != editor_) return false;
    if (e->key() == Qt::Key_Escape) {
        dismiss();
        return true;
    }
    if (entries_.isEmpty()) return false;
    switch (e->key()) {
    case Qt::Key_Up:
        move_selection(-1);
        return true;
    case Qt::Key_Down:
        move_selection(1);
        return true;
    case Qt::Key_PageUp:
        move_selection(-qMax(1, visible_rows_ - 1));
        return true;
    case Qt::Key_PageDown:
        move_selection(qMax(1, visible_rows_ - 1));
        return true;
    case Qt::Key_Return:
    case Qt::Key_Enter:
        // Ctrl+Enter runs the tab and the keypad Enter runs and clears it; neither is an acceptance.
        if (e->modifiers() & (Qt::ControlModifier | Qt::AltModifier | Qt::ShiftModifier | Qt::KeypadModifier)) return false;
        accept();
        return true;
    case Qt::Key_Tab:
        if ((e->modifiers() & ~Qt::KeypadModifier) == Qt::NoModifier) {
            accept();
            return true;
        }
        break;
    default:
        break;
    }
    return false;
}

void ScriptCompletion::move_selection(int by) {
    if (entries_.isEmpty()) return;
    selected_ = qBound(0, selected_ + by, entries_.size() - 1);
    if (selected_ < scroll_) scroll_ = selected_;
    if (selected_ >= scroll_ + visible_rows_) scroll_ = selected_ - visible_rows_ + 1;
    update();
}

void ScriptCompletion::accept() {
    if (!editor_ || selected_ < 0 || selected_ >= entries_.size()) return;
    const CompletionEntry entry = entries_[selected_];
    if (!entry.enabled) {
        dismiss();
        return;
    }
    QTextCursor c(editor_->document());
    c.setPosition(replace_start_);
    c.setPosition(replace_end_, QTextCursor::KeepAnchor);
    c.insertText(entry.insert);
    editor_->setTextCursor(c);
    dismiss();
}

void ScriptCompletion::paintEvent(QPaintEvent*) {
    QPainter p(this);
    p.setRenderHint(QPainter::Antialiasing, false);
    p.fillRect(rect(), QColor(43, 43, 43));
    p.setPen(QColor(90, 90, 90));
    p.drawRect(rect().adjusted(0, 0, -1, -1));
    int y = 1;
    if (hint_) {
        paint_hint(p, QRect(1, 1, width() - 2, kHintHeight));
        y += kHintHeight;
    }
    const int list_top = y;
    p.setFont(mono_font(9.5));
    const QFontMetrics fm(mono_font(9.5));
    for (int i = 0; i < visible_rows_; ++i) {
        const int index = scroll_ + i;
        if (index >= entries_.size()) break;
        const QRect r(1, list_top + i * kRowHeight, width() - 2, kRowHeight);
        paint_entry(p, r, entries_[index], index == selected_);
    }
    const int list_bottom = list_top + visible_rows_ * kRowHeight;
    if (!entries_.isEmpty()) {
        p.setPen(QColor(64, 64, 64));
        p.drawLine(1, list_bottom, width() - 2, list_bottom);
        const QRect detail(1, list_bottom + 1, width() - 2, qMax(0, height() - list_bottom - 2));
        const CompletionEntry& e = entries_[qBound(0, selected_, entries_.size() - 1)];
        QFont sig = mono_font(9.0);
        p.setFont(sig);
        p.setPen(QColor(0xdc, 0xdc, 0xdc));
        const QString signature = QFontMetrics(sig).elidedText(e.detail, Qt::ElideRight, detail.width() - 16);
        p.drawText(detail.adjusted(8, 3, -8, 0), Qt::AlignLeft | Qt::AlignTop, signature);
        p.setFont(mono_font(8.5));
        p.setPen(QColor(0xa8, 0xa8, 0xa8));
        const QString doc = QFontMetrics(mono_font(8.5)).elidedText(e.doc, Qt::ElideRight, detail.width() - 16);
        p.drawText(detail.adjusted(8, 16, -8, 0), Qt::AlignLeft | Qt::AlignTop, doc);
    }
    // The scrollbar, when the list is longer than what fits.
    if (visible_rows_ > 0 && entries_.size() > visible_rows_) {
        const int track_top = list_top + 1;
        const int track_h = qMax(4, visible_rows_ * kRowHeight - 2);
        const int thumb_h = qMax(12, track_h * visible_rows_ / entries_.size());
        const int max_scroll = qMax(1, static_cast<int>(entries_.size()) - visible_rows_);
        const int thumb_y = track_top + (track_h - thumb_h) * scroll_ / max_scroll;
        p.fillRect(QRect(width() - 6, track_top, 4, track_h), QColor(50, 50, 50));
        p.fillRect(QRect(width() - 6, thumb_y, 4, thumb_h), QColor(94, 94, 94));
    }
}

void ScriptCompletion::paint_entry(QPainter& p, const QRect& r, const CompletionEntry& e, bool selected) const {
    if (selected) p.fillRect(r, QColor(74, 109, 140));
    paint_icon(p, QRect(r.left() + 4, r.top() + 2, 14, 14), e.kind, e.enabled, selected);
    const QFont f = mono_font(9.5);
    p.setFont(f);
    const QFontMetrics fm(f);
    const int cat_w = fm.horizontalAdvance(e.category);
    const int label_x = r.left() + 24;
    const int label_w = qMax(0, r.width() - 28 - cat_w - 8);
    p.setPen(selected ? QColor(0xff, 0xff, 0xff) : (e.enabled ? QColor(0xd2, 0xd2, 0xd2) : QColor(0x78, 0x78, 0x78)));
    p.drawText(QRect(label_x, r.top(), label_w, r.height()), Qt::AlignLeft | Qt::AlignVCenter, fm.elidedText(e.label, Qt::ElideRight, label_w));
    p.setPen(selected ? QColor(0xcf, 0xe2, 0xf0) : QColor(0x8f, 0x8f, 0x8f));
    p.drawText(QRect(r.right() - cat_w - 6, r.top(), cat_w, r.height()), Qt::AlignRight | Qt::AlignVCenter, e.category);
}

void ScriptCompletion::paint_icon(QPainter& p, const QRect& r, CompletionEntry::Kind kind, bool enabled, bool selected) const {
    QColor c = colour_for(kind);
    if (!enabled) c = QColor(0x70, 0x70, 0x70);
    QColor bg = c;
    bg.setAlpha(selected ? 70 : 38);
    p.setPen(Qt::NoPen);
    p.setBrush(bg);
    p.drawRoundedRect(QRectF(r).adjusted(0, 1, 0, -1), 2, 2);
    p.setPen(c);
    QFont f = mono_font(7.5);
    f.setBold(true);
    p.setFont(f);
    p.drawText(r, Qt::AlignCenter, glyph_for(kind));
}

void ScriptCompletion::paint_hint(QPainter& p, const QRect& r) const {
    p.save();
    p.setClipRect(r.adjusted(0, 0, -2, 0));
    p.setFont(mono_font(9.0));
    const QFontMetrics fm(mono_font(9.0));
    int x = r.left() + 8;
    const int y = r.top() + r.height() / 2 + fm.ascent() / 2 - 1;
    auto draw = [&](const QString& text, const QColor& colour, bool bold) {
        QFont f = mono_font(9.0);
        f.setBold(bold);
        p.setFont(f);
        p.setPen(colour);
        p.drawText(x, y, text);
        x += QFontMetrics(f).horizontalAdvance(text);
    };
    draw(hint_name_ + "(", QColor(0x9a, 0x9a, 0x9a), false);
    for (int i = 0; i < hint_params_.size(); ++i) {
        if (i) draw(QStringLiteral(", "), QColor(0x9a, 0x9a, 0x9a), false);
        const bool current = i == hint_current_;
        draw(hint_params_[i], current ? QColor(0xff, 0xff, 0xff) : QColor(0x8a, 0x8a, 0x8a), current);
    }
    draw(QStringLiteral(")"), QColor(0x9a, 0x9a, 0x9a), false);
    p.setPen(QColor(82, 82, 82));
    p.drawLine(r.left() + 1, r.bottom(), r.right() - 1, r.bottom());
    p.restore();
}

int ScriptCompletion::row_at(const QPoint& pos) const {
    const int y = pos.y() - 1 - hint_height();
    if (y < 0) return -1;
    const int row = y / kRowHeight;
    if (row < 0 || row >= visible_rows_) return -1;
    const int index = scroll_ + row;
    return index < entries_.size() ? index : -1;
}

void ScriptCompletion::mouseMoveEvent(QMouseEvent* e) {
    const int row = row_at(e->position().toPoint());
    if (row >= 0 && row != selected_) {
        selected_ = row;
        update();
    }
}

void ScriptCompletion::mousePressEvent(QMouseEvent* e) {
    if (e->button() != Qt::LeftButton) return;
    const int row = row_at(e->position().toPoint());
    if (row < 0) return;
    selected_ = row;
    accept();
}

void ScriptCompletion::wheelEvent(QWheelEvent* e) {
    if (entries_.isEmpty()) return;
    move_selection(e->angleDelta().y() < 0 ? 3 : -3);
    e->accept();
}

bool ScriptCompletion::eventFilter(QObject* o, QEvent* e) {
    if (o == host_) {
        if (e->type() == QEvent::Resize && isVisible()) place();
        if (e->type() == QEvent::Hide || e->type() == QEvent::WindowDeactivate) dismiss();
    }
    return QWidget::eventFilter(o, e);
}

} // namespace az
