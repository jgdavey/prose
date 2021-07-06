# prose

[![Build Status](https://github.com/jgdavey/prose/actions/workflows/ci.yml/badge.svg)](https://github.com/jgdavey/prose/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/prose)](https://crates.io/crates/prose)
[![Crates.io](https://img.shields.io/crates/v/prose)](https://crates.io/crates/prose)

A CLI utility to reformat text.

`prose` is inspired by [par](http://www.nicemice.net/par), and aims to be similar, albeit with fewer features, while also being Unicode-aware.


## Installation


### Via pre-built binary release

Download the [latest release](https://github.com/jgdavey/prose/releases/latest) for your platform, extract, and put the `prose` binary wherever you'd like.

NOTE that on recent versions of Mac OS, you may need to un-quarantine the downloaded binary for it to run. To do so:

```shell
xattr -d com.apple.quarantine path/to/prose
```


### Via cargo

If you're a Rust programmer, prose can be installed with cargo. Note that the minimum supported version of Rust for prose is 1.38.0, although it may work with older versions.

```shell
cargo install prose
```


## Examples

Given the following input, a [quote by Angela Davis](https://www.latimes.com/opinion/op-ed/la-oe-morrison-davis-20140507-column.html), on STDIN:

    I believe profoundly in the possibilities of democracy, but democracy needs
    to be emancipated from capitalism. As long as we inhabit
    a capitalist democracy, a future of
    racial equality, gender equality, economic equality will elude us.

let's look at some invocations and their output.

As a baseline, we can view what a standard `fmt` would yield.

```shell
fmt -44
```

    I believe profoundly in the possibilities of
    democracy, but democracy needs to be
    emancipated from capitalism. As long as we
    inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic
    equality will elude us.

Using the same width option, `prose` yields a less jagged output.

```shell
prose -w 44
```

    I believe profoundly in the possibilities
    of democracy, but democracy needs to be
    emancipated from capitalism. As long as we
    inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic
    equality will elude us.

Using the same width option, `prose` yields a less jagged output.


### Prefixes and suffixes (like code comments)

`prose` attempts to keep prefix/suffix line adornment&#x2013;such as borders, code comment marks, and indentation&#x2013;intact.

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

```shell
prose -w 44 -l
```

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

```shell
prose -w 60
```

        I believe profoundly in the possibilities of democracy,
    but democracy needs to be emancipated from capitalism. As
    long as we inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic equality will
    elude us.


### Email quotes

`prose` attempts to be smart about keeping email quoting levels, if they are the recognizable format of `>` introducing each new quote level. With the following example email input:

    Abby,
    
    Whoops! Sorry about that. I forgot to export it as a PDF. Re-attaching
    to this message. Again, let me know if you need anything else!
    
    Bill
    
    Abby writes:
    
    > Bill,
    > 
    > I tried to open the attachment, but it looks like it isn't in the correct format. Could
    > you re-send it?
    > 
    > Thanks,
    > 
    > Abby
    > 
    > Bill writes:
    > 
    > > Abby,
    > >
    > > I like the way this project is turning out so far! Kudos.
    > > 
    > > I'm attaching my first pass at working version. Let me know if you need anything else
    > > before our meeting on Tuesday.
    > > 
    > > Cheers,
    > > 
    > > Bill
    > 
    >
    > This message and its contents are confidential or whatever.
    > 

And a just the width set:

```shell
prose -w 48
```

    Abby,
    
    Whoops! Sorry about that. I forgot to export it
    as a PDF. Re-attaching to this message. Again,
    let me know if you need anything else!
    
    Bill
    
    Abby writes:
    
    > Bill,
    >
    > I tried to open the attachment, but it looks
    > like it isn't in the correct format. Could you
    > re-send it?
    >
    > Thanks,
    >
    > Abby
    >
    > Bill writes:
    >
    > > Abby,
    > >
    > > I like the way this project is turning out
    > > so far! Kudos.
    > >
    > > I'm attaching my first pass at working
    > > version. Let me know if you need anything
    > > else before our meeting on Tuesday.
    > >
    > > Cheers,
    > >
    > > Bill
    >
    > This message and its contents are confidential
    > or whatever.
    >


### Using the fit option, "-f"

Without `-f`, the result works, but could look slightly less jagged on the ends.

```shell
prose -w 58
```

    I believe profoundly in the possibilities of democracy,
    but democracy needs to be emancipated from capitalism. As
    long as we inhabit a capitalist democracy, a future of
    racial equality, gender equality, economic equality will
    elude us.

A width parameter of 56 or so would get us there, and if we are okay having the longest possible maximum line length be less than the target (width) we specify, we can pass `-f`.

```shell
prose -w 58 -f
```

    I believe profoundly in the possibilities of democracy,
    but democracy needs to be emancipated from capitalism.
    As long as we inhabit a capitalist democracy, a future
    of racial equality, gender equality, economic equality
    will elude us.


### With markdown input

Take, for example, this portion of the Contributor Covenant Code of Conduct:

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

```shell
prose -w 42
```

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

Notice how the bulleted list has been run together. To leave bulleted lists and other formatting intact, use the `-m` or `--markdown` switch. Doing so will interpret the input as markdown, only formatting plain paragraphs.

```shell
prose -w 42 --markdown
```

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

Future versions may improve on this by indenting bulleted lists more intelligently.


## License

Licensed under either of:

-   Apache License, Version 2.0, (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

at your option.
