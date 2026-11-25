Feature: contract

  The template's own Prism application, in app/: given the locked and the
  observed BLAKE3 label of each universal policy file, it answers whether the
  repository conforms to the template contract, and which files drifted.

  @TC-01 @build
  Scenario: The checker answers conformant exactly when every universal policy file's observed label equals its locked label
    Given the contract model in app/src/Contract.lex.tex
    When prism-code check has Lean's kernel prove Contract.conformant_decides
    Then conformant is true exactly when each of the three observed labels equals its locked label
    And the verdict the application returns is decided by that same conformant

  @TC-02 @build @rejects
  Scenario: The checker answers invalid request to a request that is neither exactly one locked and one observed label for each universal policy file nor a Studio guard or stage-bindings request shorter than 192 bytes, and no request is read as both
    Given the contract model in app/src/Contract.lex.tex
    When prism-code check has Lean's kernel prove Contract.accepts_exactly_a_request and Contract.forms_are_disjoint
    Then a request is read as pins exactly when it is 192 bytes, three pins of two labels each
    And a request is read as a Studio guard or stage-bindings request only when it is shorter than 192 bytes, so never as both
    And prism-code build replays requests of 0, 191, 193 and 256 bytes and a Studio request with an unknown operation, each answered invalid request
