Feature: gui-instance

  The instances journey of the Studio UI Prism, in app/src/Studio/Gui/Instance.lex.tex. The app answers
  studio TAB gui TAB instance TAB view TAB VIEWER TAB PRISM-ID TAB HOST... with the journey, its view and its action.

  @VS-113 @build
  Scenario: Studio.Gui.Instance presents the instances journey of the Studio UI Prism in Stitch Product.Experience's pattern: the instances of a Prism ID on a runtime host other than the github mirror (Studio.Instance, VE-77) show in the instancesHosted view, and their owner has the sync action enabled (VE-24)
    Given the instances journey in app/src/Studio/Gui/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Instance.the_owner_sees_its_hosted_instances
    Then prism:alex with instances on github, a browser and a phone sees instancesHosted with sync enabled
    And prism-code build replays that request, answered with that view

  @VS-114 @build
  Scenario: The instances of a Prism ID synchronize only under that Prism ID (Studio.Instance), so Studio.Gui.Instance shows them to any other Prism ID with sync disabled
    Given the instances journey in app/src/Studio/Gui/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Instance.another_prism_id_cannot_synchronize
    Then prism:bob viewing prism:alex's instances sees sync disabled
    And prism-code build replays that request, answered with that view

  @VS-115 @build
  Scenario: GitHub is only a mirror, never a runtime, so Studio.Gui.Instance presents instances on github alone in the instancesMirrorOnly view with the hosting refusal of Studio.Instance, sync disabled
    Given the instances journey in app/src/Studio/Gui/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Instance.shows_instances_on_the_mirror_alone
    Then prism:alex with only github sees instancesMirrorOnly with sync disabled
    And prism-code build replays that request, answered with that view

  @VS-116 @build
  Scenario: The sync action of the instances journey is guarded, so Studio.Gui.Instance never enables it for a viewer that is not the instances' own Prism ID, wherever they are hosted
    Given the instances journey in app/src/Studio/Gui/Instance.lex.tex
    When prism-code check has Lean's kernel prove Studio.Gui.Instance.only_the_owner_synchronizes
    Then sync is disabled for any other viewer
