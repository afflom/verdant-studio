Feature: shared-catalog

  Catalogs of shared content, in app/src/Studio/SharedCatalog.lex.tex. The app answers studio TAB shared TAB
  publish TAB CATALOG TAB OWNER TAB shared|private, subscribe TAB CATALOG TAB PRISMID, permission TAB CATALOG
  TAB REQUESTER TAB SCOPE and discover TAB CATALOG [TAB VIA TAB LISTED...] with its answer, or a refusal.

  @VS-50 @build
  Scenario: Studio.SharedCatalog publishes a person's shared content in a catalog, distributed to every instance subscribed to that catalog
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.publishes_shared_content
    Then shared content of prism:alex is published in creator-games
    And prism-code build replays that request, answered with that answer

  @VS-51 @build @rejects
  Scenario: Content is private to its Prism ID until it is shared, so Studio.SharedCatalog refuses to publish unshared content in a catalog, naming the Prism ID and the catalog
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.refuses_to_publish_private_content
    Then private content of prism:alex is answered with the privacy refusal
    And prism-code build replays that request, answered with that refusal

  @VS-52 @build
  Scenario: Studio.SharedCatalog subscribes every instance of a Prism ID to a catalog, so its shared content reaches each of them, synchronized under that Prism ID
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.subscribes_every_instance_of_a_prism_id
    Then prism:bob is subscribed to creator-games on every one of its instances
    And prism-code build replays that request, answered with that answer

  @VS-53 @build
  Scenario: A catalog has maintainers as a project does, so Studio.SharedCatalog hands a request to maintain, publish to, curate or moderate a catalog on to Verdant Engine's VE-70 signing-request scheme for the catalog's maintainers to decide
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.hands_a_catalog_permission_request_to_the_engine
    Then prism:bob's request to curate creator-games is handed to VE-70
    And prism-code build replays that request, answered with that answer

  @VS-54 @build @rejects
  Scenario: Studio.SharedCatalog refuses a permission request for any scope other than maintain, publish, curate or moderate, naming the catalog and the scope
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.refuses_a_scope_a_catalog_does_not_grant
    Then a request for delete is answered with the scope refusal
    And prism-code build replays that request, answered with that refusal

  @VS-55 @build
  Scenario: Verdant Studio itself is the one catalog Studio.SharedCatalog finds with no catalog to find it through
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.verdant_studio_is_discoverable_everywhere
    Then Verdant Studio is found with no catalog named
    And prism-code build replays that request, answered with that answer

  @VS-56 @build @rejects
  Scenario: Only Verdant Studio itself is globally discoverable, so Studio.SharedCatalog refuses to find any other catalog, the Verdant Studio project included, except through a catalog that lists it, and pins none
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.refuses_global_discovery_of_any_other_catalog
    Then the verdant-studio project, like any project, is not found without a catalog that lists it
    And prism-code build replays that request, answered with that refusal

  @VS-57 @build
  Scenario: Studio.SharedCatalog finds a catalog through another catalog that lists it
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.finds_a_catalog_through_one_that_lists_it
    Then creator-games is found through Verdant Studio, which lists it
    And prism-code build replays that request, answered with that answer

  @VS-58 @build @rejects
  Scenario: Studio.SharedCatalog refuses to find a catalog through one that does not list it, naming both
    Given the shared catalog model in app/src/Studio/SharedCatalog.lex.tex
    When prism-code check has Lean's kernel prove Studio.SharedCatalog.refuses_a_catalog_through_one_that_does_not_list_it
    Then creator-games is not found through other-catalog, which does not list it
    And prism-code build replays that request, answered with that refusal
