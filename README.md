Librsvg
=======

This is librsvg - A small SVG rendering library associated with the
GNOME Project.

Do you want to render non-animated SVGs to a Cairo surface with a
minimal, no-nonsense API?  Librsvg may be adequate for you.

There is a code of conduct for contributors to librsvg; please see the
file `code_of_conduct.md`.

For information on how to report bugs, or how to contribute to librsvg
in general, please see the file `CONTRIBUTING.md`.

Goals of librsvg
----------------

Librsvg aims to be a low-footprint library for rendering SVG images.
It is used primarily in the [GNOME project](https://www.gnome.org) to render
SVG icons and vector images that appear on the desktop.  It is also
used in Wikimedia to render the SVG images that appear in Wikipedia,
so that even old web browsers can display them.

We aim to be a "render this SVG for me, quickly, and with a minimal
API" kind of library.  The SVG specification is huge, and definitely
contains features that are not frequently used in the Real World, if
at all.

Feature additions will be considered on a case-by-case basis.  Extra
points if you provide a proof-of-concept patch, and an example of the
situation in which you encountered that missing feature!

Non-goals of librsvg
--------------------

We don't aim to:

* Implement every single SVG feature that is in the spec.

* Implement external access to the SVG's DOM.

* Implement support for CSS-based animations (but if you can think of
  a nice API to do this, we'd be glad to know!)

* Replace the industrial-strength SVG rendering machinery in modern
  web browsers.
