Feature: typeface

  The typeface generator Studio.Generator.Typeface, in
  app/src/Studio/Generator/Typeface/: a Prism's font/otf facet, whose glyphs
  are Computer Modern's METAFONT programs run with METAFONT's own arithmetic
  and written as CFF outlines.

  @VS-81 @build
  Scenario: The typeface generator's arithmetic is METAFONT's, as mf.web 2.71828182 computes it: make_fraction, take_fraction, make_scaled, take_scaled, square_rt, pyth_add, n_arg and n_sin_cos give mf.web's results bit for bit, rounding and overflow to el_gordo included
    Given the arithmetic model in app/src/Studio/Generator/Typeface/Scaled.lex.tex
    When prism-code check has Lean's kernel prove Studio.Generator.Typeface.Scaled.makes_fractions_as_mf_web_does and Studio.Generator.Typeface.Scaled.takes_fractions_as_mf_web_does and Studio.Generator.Typeface.Scaled.makes_scaled_as_mf_web_does and Studio.Generator.Typeface.Scaled.takes_scaled_as_mf_web_does and Studio.Generator.Typeface.Scaled.roots_as_mf_web_does and Studio.Generator.Typeface.Scaled.measures_angles_as_mf_web_does and Studio.Generator.Typeface.Scaled.adds_pythagorean_as_mf_web_does and Studio.Generator.Typeface.Scaled.sines_and_cosines_as_mf_web_does
    Then each routine answers the value mf.web's own routine computes for the same integers
