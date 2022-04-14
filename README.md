
# Android-Strings-Auto-Translate-Tool

Tool to automate my handmade work regarding translation android strings with google translator web service

Made by learning Rust language (so many possible errors and situations are unhandled).

**Usage:**

    auto_trans [path] [lang]

**Where:**

[path] - path to "res" folder
[lang] - language code to translate from, means values/strings.xml file should contain strings in this language

The application looks through the folders inside [res] and finds the file values/strings.xml, which is considered the main file with strings in [lang] language. Then it looks through the folders inside [res] which match the pattern values-xx (where xx is the language code from the list of supported GT languages) with strings.xml inside.
The application then translates any strings from main strings.xml that are *NOT YET* translated to other languages using HTML-requests to translate.google.com.

This was inspired by the https://github.com/Ra-Na/GTranslate-strings-xml/tree/master/GoogleTranslate repository, but unlike it, my utility does not translate absolutely all strings every time and is convenient to use directly in the android- project.