Feature: runtime

  The release cut of Verdant Studio's own runtime, in app/src/Studio/Runtime.lex.tex.
  The app answers a release request, studio TAB release TAB REVISION TAB VERSION TAB
  INCREMENT TAB GATE..., each gate NAME:passed when it passed, with the SemVer 2.0.0
  release it cuts from that revision, or the refusal that says why none is cut.

  @VS-33 @build
  Scenario: The release cut of Studio's runtime, Studio.Runtime, increments a SemVer 2.0.0 normal version X.Y.Z as its rules 6 to 8 say: a major increment resets minor and patch to 0, a minor increment resets patch to 0, and every increment gives the new version a higher precedence than the old one by rule 11, so a release is cut from every revision on which each gate passed
    Given the release cut in app/src/Studio/Runtime.lex.tex
    When prism-code check has Lean's kernel prove Studio.Runtime.next_has_higher_precedence, Studio.Runtime.next_resets_the_lower_numbers, Studio.Runtime.cuts_a_minor_release_when_every_gate_passed and Studio.Runtime.a_major_release_resets_minor_and_patch
    Then every increment of every version has a higher precedence than the version it increments, a major increment resets minor and patch to 0 and a minor increment resets patch to 0
    And prism-code build replays the request to cut a minor release of 0.1.0 from revision c587c01 on which check and build passed, answered release 0.2.0 of revision c587c01

  @VS-34 @build @rejects
  Scenario: The release cut refuses a revision on which a gate did not pass, naming the gate and the revision, so that no release is cut from a revision that fails any check
    Given the release cut in app/src/Studio/Runtime.lex.tex
    When prism-code check has Lean's kernel prove Studio.Runtime.refuses_a_failed_gate
    Then a request whose gate export did not pass is answered with the refusal that names it
    And prism-code build replays the request to cut a patch release of revision c587c01 whose export failed, answered with that refusal

  @VS-35 @build @rejects
  Scenario: The release cut refuses a revision on which no gate ran, since every gate passing is only established by a gate that ran
    Given the release cut in app/src/Studio/Runtime.lex.tex
    When prism-code check has Lean's kernel prove Studio.Runtime.refuses_a_revision_no_gate_ran_on
    Then a request that names no gate is answered with the refusal that names its revision
    And prism-code build replays the request to cut a patch release of revision c587c01 with no gate, answered with that refusal

  @VS-36 @build @rejects
  Scenario: The release cut refuses a current version that is not a SemVer 2.0.0 normal version X.Y.Z of non-negative integers with no leading zeroes, naming it
    Given the release cut in app/src/Studio/Runtime.lex.tex
    When prism-code check has Lean's kernel prove Studio.Runtime.refuses_a_leading_zero
    Then the version 01.1.0 is answered with the refusal that names it
    And prism-code build replays the request to cut a patch release of 01.1.0, answered with that refusal

  @VS-37 @build @rejects
  Scenario: The release cut refuses an increment that is not major, minor or patch, the three increments of SemVer 2.0.0, naming it
    Given the release cut in app/src/Studio/Runtime.lex.tex
    When prism-code check has Lean's kernel prove Studio.Runtime.refuses_an_unknown_increment
    Then the increment feature is answered with the refusal that names it
    And prism-code build replays the request to cut a feature release of 0.1.0, answered with that refusal
