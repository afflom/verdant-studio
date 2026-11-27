Feature: instance

  A person's Verdant instances, in app/src/Studio/Instance.lex.tex.
  The app answers studio TAB instance TAB host TAB PRISMID TAB HOST..., studio TAB instance TAB sync
  TAB PRISMID TAB PRISMID and studio TAB instance TAB read TAB OWNER TAB REQUESTER TAB shared|private
  with the request it hands to Verdant Engine, the read it allows, or a refusal.

  @VS-43 @build
  Scenario: Studio.Instance hands a hosting plan with a host other than the mirror on to Verdant Engine's multi-gateway hosting, VE-77, as the typed request naming the Prism ID whose instances it hosts, with GitHub a mirror only
    Given the instance model in app/src/Studio/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Instance.hands_a_hosting_plan_with_a_runtime_host_to_the_engine
    Then of the hosts github, browser and phone for prism:alex, the browser and phone are runtime hosts, so the plan is handed to VE-77
    And prism-code build replays that request, answered with that answer

  @VS-44 @build @rejects
  Scenario: GitHub is only a mirror, never a runtime, so Studio.Instance refuses a hosting plan whose only host is github, naming the Prism ID
    Given the instance model in app/src/Studio/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Instance.refuses_a_hosting_plan_on_the_mirror_alone
    Then a plan for prism:alex on github alone is answered with the mirror refusal
    And prism-code build replays that request, answered with that refusal

  @VS-45 @build
  Scenario: Studio.Instance hands a synchronization of two instances under one Prism ID on to Verdant Engine's Prism ID signed input journal batches, VE-24, naming that Prism ID
    Given the instance model in app/src/Studio/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Instance.hands_a_sync_under_one_prism_id_to_the_engine
    Then two instances of prism:alex are handed to VE-24 to synchronize
    And prism-code build replays that request, answered with that answer

  @VS-46 @build @rejects
  Scenario: Instances synchronize only under one Prism ID, so Studio.Instance refuses to synchronize instances of two Prism IDs, naming both and the catalog through which shared content crosses
    Given the instance model in app/src/Studio/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Instance.refuses_a_sync_across_prism_ids
    Then an instance of prism:bob and an instance of prism:alex are answered with the synchronization refusal
    And prism-code build replays that request, answered with that refusal

  @VS-47 @build
  Scenario: A person's data is readable by their own Prism ID, and by any Prism ID once shared, by Studio.Instance under the ISO/IEC 29100 principle of consent and choice
    Given the instance model in app/src/Studio/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Instance.the_owner_reads_its_own_data and Studio.Instance.anyone_reads_shared_data
    Then prism:alex reads its own private data, and prism:bob reads the shared data of prism:alex
    And prism-code build replays that request, answered with that answer

  @VS-48 @build @rejects
  Scenario: A person's data is private to their Prism ID until they share it, so Studio.Instance refuses another Prism ID's read of unshared data, naming both (ISO/IEC 29100 consent and choice)
    Given the instance model in app/src/Studio/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Instance.refuses_to_read_private_data_of_another_prism_id
    Then prism:bob's read of the private data of prism:alex is answered with the privacy refusal
    And prism-code build replays that request, answered with that refusal

  @VS-49 @open
  Scenario: Verdant Engine's VE-77 multi-gateway hosting and VE-24 Prism ID signed journal synchronization are callable from Verdant Studio, so that the requests Studio.Instance hands on are carried out by the engine; REQ-DISTRIBUTED-INSTANCES depends on them, and until the engine exposes them Studio holds the requests it would send
    Given the requests of VS-43 and VS-45
    When Verdant Engine exposes VE-77 and VE-24 to Verdant Studio
    Then the engine hosts and synchronizes the instances, which Studio does not do itself
