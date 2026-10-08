# Saga: private-names

Goal: follow X_eTaL's new h: namespace (private file-local names,
PN1-PN7): library .xtl and .xtlm files no longer leave helper
functions bare; app .xtl files may use h: too, though their u:
functions (the program's "stages") stay u:.

1. repin: pin X_eTaL main (the commit with PN1/PN4/PN5/PN7
   implemented) in XETAL_COMMIT; goldens, web, browser, bench-check.
2. migrate-h: Stencil.xtlm's and Pipes.xtl's bare private helper
   functions (and Pipes.xtl's bare private variables) become h:;
   every call site updated; a CLAUDE.md rule records the convention;
   docs updated where they show these names.
