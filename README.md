

# prose

[![Build Status](https://travis-ci.org/jgdavey/prose.svg?branch=master)](https://travis-ci.org/jgdavey/prose)
[![Crates.io](https://img.shields.io/crates/l/prose)](https://crates.io/crates/prose)
[![Crates.io](https://img.shields.io/crates/v/prose)](https://crates.io/crates/prose)

A CLI utility to reformat text.

`prose` is inspired by [par](http://www.nicemice.net/par), and aims to be similar, albeit with fewer
features, while also being Unicode-aware.


## Installation


### Via pre-built binary release

Download the [latest release](https://github.com/jgdavey/prose/releases/latest) for your platform, extract, and put the
`prose` binary wherever you'd like.


### Via cargo

If you're a Rust programmer, prose can be installed with cargo. Note
that the minimum supported version of Rust for prose is 1.38.0,
although it may work with older versions.

    cargo install prose


## Examples

Given the following input on STDIN:

    I believe profoundly in the possibilities of democracy, but democracy needs
    to be emancipated from capitalism. As long as we inhabit
    a capitalist democracy, a future of
    racial equality, gender equality, economic equality will elude us.

let's look at some invocations and their output.

As a baseline, we can view what a standard `fmt` would yield.

    fmt -44

    I believe profoundly in the possibilities of
    democracy, but democracy needs to be
    emancipated from capitalism. As long as we
    inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic
    equality will elude us.

Using the same width option, `prose` yields a less jagged output.

    prose -w 44

    I believe profoundly in the possibilities
    of democracy, but democracy needs to be
    emancipated from capitalism. As long as we
    inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic
    equality will elude us.

Using the same width option, `prose` yields a less jagged output.


### Prefixes and suffixes (like code comments)

`prose` attempts to keep prefix/suffix line adornment&#x2013;such as
borders, code comment marks, and indentation&#x2013;intact.

Consider the the following input:

    /* Lorem ipsum dolor sit ameÞ, sea ðicat instructíor deterrúisset cu, ex */
    /* graéce scriptæ minimum eós. Nö qui plæcérat eloquentiam, alíenum */
    /* sæluÞæÞus evertitur éam nö, mucíus tibíque ad esÞ. Atqui fêugâit eum */
    /* id. Síngulis, id meí chöro essent. Íð æssentior moderatius intellégam */
    /* næm, solet abhorreant ne cum. Qúod æssum intellegam ad vím, pró diam */
    /* vítae nónumy ei, cúm inaní primís té. */
    /*  */
    /* Ex dicunt åssentiør eum, ad nusquam definiebæs qui, */
    /* vidit åtqui per ut. Qui ut simul dicunt */
    /* sålutændi. Dūō scaevōla vulputaÞe ut. Quō volupÞua rætiōnibus uÞ, et */
    /* postulǽnt intellegǽt vitupērata vim. Primis legimus haȝemus sit æð, */
    /* seæ hǣrum, fâcilisi, êum ôfficiis iudicabit âd. */

And the output:

    prose -w 44 -l

    /* Lorem ipsum dolor sit ameÞ, sea        */
    /* ðicat instructíor deterrúisset cu,     */
    /* ex graéce scriptæ minimum eós. Nö      */
    /* qui plæcérat eloquentiam, alíenum      */
    /* sæluÞæÞus evertitur éam nö, mucíus     */
    /* tibíque ad esÞ. Atqui fêugâit eum      */
    /* id. Síngulis, id meí chöro essent.     */
    /* Íð æssentior moderatius intellégam     */
    /* næm, solet abhorreant ne cum. Qúod     */
    /* æssum intellegam ad vím, pró diam      */
    /* vítae nónumy ei, cúm inaní primís té.  */
    /*                                        */
    /* Ex dicunt åssentiør eum, ad nusquam    */
    /* definiebæs qui, vidit åtqui per        */
    /* ut. Qui ut simul dicunt sålutændi.     */
    /* Dūō scaevōla vulputaÞe ut. Quō         */
    /* volupÞua rætiōnibus uÞ, et postulǽnt   */
    /* intellegǽt vitupērata vim. Primis      */
    /* legimus haȝemus sit æð, seæ hǣrum,     */
    /* fâcilisi, êum ôfficiis iudicabit âd.   */

It similarly attempts to preserve paragraph indentation:

Given this indented input:

        I believe profoundly in the possibilities of democracy, but democracy needs
    to be emancipated from capitalism. As long as we inhabit
    a capitalist democracy, a future of
    racial equality, gender equality, economic equality will elude us.

    prose -w 60

        I believe profoundly in the possibilities of democracy,
    but democracy needs to be emancipated from capitalism. As
    long as we inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic equality will
    elude us.


### Using the fit option, "-f"

Without `-f`, the result works, but could look slightly less jagged on
the ends.

    prose -w 58

    I believe profoundly in the possibilities of democracy,
    but democracy needs to be emancipated from capitalism. As
    long as we inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic equality will
    elude us.

A width parameter of 56 or so would get us there, and if we are okay
having the longest possible maximum line length be less than the
target (width) we specify, we can pass `-f`.

    prose -w 58 -f

    I believe profoundly in the possibilities of democracy,
    but democracy needs to be emancipated from capitalism.
    As long as we inhabit a capitalist democracy, a future
    of racial equality, gender equality, economic equality
    will elude us.


### With markdown input

Take, for example, this portion of the Contributor Covenant Code of
Conduct:

    ## Our Pledge
    
    In the interest of fostering an open and welcoming environment, we as
    contributors and maintainers pledge to making participation in our project and
    our community a harassment-free experience for everyone, regardless of age, body
    size, disability, ethnicity, sex characteristics, gender identity and expression,
    level of experience, education, socio-economic status, nationality, personal
    appearance, race, religion, or sexual identity and orientation.
    
    ## Our Standards
    
    Examples of behavior that contributes to creating a positive environment
    include:
    
    * Using welcoming and inclusive language
    * Being respectful of differing viewpoints and experiences
    * Gracefully accepting constructive criticism
    * Focusing on what is best for the community
    * Showing empathy towards other community members

With a standard invocation:

    prose -w 42

    ## Our Pledge
    
    In the interest of fostering an open
    and welcoming environment, we as
    contributors and maintainers pledge to
    making participation in our project
    and our community a harassment-free
    experience for everyone, regardless of
    age, body size, disability, ethnicity,
    sex characteristics, gender identity
    and expression, level of experience,
    education, socio-economic status,
    nationality, personal appearance,
    race, religion, or sexual identity and
    orientation.
    
    ## Our Standards
    
    Examples of behavior that contributes to
    creating a positive environment include:
    
    * Using welcoming and inclusive language
    * Being respectful of differing viewpoints
    * and experiences Gracefully accepting
    * constructive criticism Focusing on
    * what is best for the community Showing
    * empathy towards other community members

Notice how the bulleted list has been run together. To leave bulleted
lists and other formatting intact, use the `-m` or `--markdown`
switch. Doing so will interpret the input as markdown, only formatting
plain paragraphs.

    prose -w 42 --markdown

    ## Our Pledge
    
    In the interest of fostering an open
    and welcoming environment, we as
    contributors and maintainers pledge to
    making participation in our project
    and our community a harassment-free
    experience for everyone, regardless of
    age, body size, disability, ethnicity,
    sex characteristics, gender identity
    and expression, level of experience,
    education, socio-economic status,
    nationality, personal appearance,
    race, religion, or sexual identity and
    orientation.
    
    ## Our Standards
    
    Examples of behavior that contributes to
    creating a positive environment include:
    
    * Using welcoming and inclusive language
    * Being respectful of differing viewpoints and experiences
    * Gracefully accepting constructive criticism
    * Focusing on what is best for the community
    * Showing empathy towards other community members

Future versions may improve on this by indenting bulleted lists more
intelligently.


## License

Licensed under either of:

-   Apache License, Version 2.0, (LICENSE-APACHE or
    <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

at your option.

