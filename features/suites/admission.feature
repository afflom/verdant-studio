Feature: admission

  Verdant Studio admits a construct only with one citation, Std, Req or Der,
  and only through its own API (D8, D12). The checks are proved over register
  indices in app/src/Studio/Admission.lex.tex and run by `verdant_studio check`.

  @VS-01 @build
  Scenario: A requirement citation resolves exactly when its requirement is in the requirements register
    Given the admission model in app/src/Studio/Admission.lex.tex
    When prism-code check has Lean's kernel prove Studio.Admission.req_resolves_exactly_when_registered
    Then a Req citation resolves exactly when its requirement index is a member of the requirements register

  @VS-02 @build
  Scenario: A standard citation resolves exactly when its authority is registered and its catalog row, if it names one, is registered
    Given the admission model in app/src/Studio/Admission.lex.tex
    When prism-code check has Lean's kernel prove Studio.Admission.std_resolves_exactly_when_cited_authority_registered
    Then a Std citation resolves exactly when its authority is registered and its row resolves

  @VS-03 @build
  Scenario: A derivation citation resolves exactly when every input is already admitted and its check is a registered ID
    Given the admission model in app/src/Studio/Admission.lex.tex
    When prism-code check has Lean's kernel prove Studio.Admission.der_resolves_exactly_when_inputs_admitted
    Then a Der citation resolves exactly when its inputs are admitted and its check is in model/ids.toml

  @VS-04 @build
  Scenario: Admission walks the declarations in order: the first whose citation does not resolve is refused, and each admitted one becomes an input for those after it
    Given the admission model in app/src/Studio/Admission.lex.tex
    When prism-code check has Lean's kernel prove Studio.Admission.first_decl_decides_admission
    Then each admitted construct becomes an input for the declarations after it
    And verdant_studio check answers "unresolved citation" with the first construct that does not resolve
    And verdant_studio check answers "unbound constraint" with the first requirement whose constraint evidence names an authority or catalog row that is not bound

  @VS-05 @build
  Scenario: The modules are grounded only when every module has a stratum row, every stratum row names a module, and every stratum is at most 7
    Given the admission model in app/src/Studio/Admission.lex.tex
    When prism-code check has Lean's kernel prove Studio.Admission.module_grounded_only_when_declared
    Then verdant_studio check refuses a module without a row in model/strata.json

  @VS-06 @build
  Scenario: The registers are monotone only when every locked entry is still present
    Given the admission model in app/src/Studio/Admission.lex.tex
    When prism-code check has Lean's kernel prove Studio.Admission.monotone_keeps_every_locked_entry
    Then verdant_studio check refuses a register that dropped an entry of model/register-lock.json

  @VS-07 @some-true
  Scenario: An authority's committed artifact is refused when its bytes are not the checksum its row records
    Given an authority row of model/authorities.toml that names an artifact
    When prism-code check recomputes its SHA-256
    Then a mismatch is refused with contract.artifact
