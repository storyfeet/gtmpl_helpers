# Gtmpl Helpers

The point in this Library is to provide useful helper methods for cases where you really do need the templates to be capable of more than just printing exactly what it's given.

My personal use case was to be able to generate specific svg layouts from numbers in cards which meant multiplying those numbers by the size of the card.
files. This meant doing more maths in the template than is common.
I've broken up the helpers roughly by subject so it should be easy to find the ones you need.

The simplest way to use this is to import the trait and call ```with_all()``` on a template.


```rust
use gtmpl::{Template,Context};
use gtmpl_value::{Value,Number};

use gtmpl_helpers::THelper;
let mut t = Template::default().with_all();
t.parse(r#"<rect {{xywh (mul . 5) (add . 11) 40 20 "px"}}/>"#);

let s = t.q_render(4).unwrap();
assert_eq!(s,r#"<rect x="20px" y="15px" width="40px" height="20px" />"#.to_string())

 ```

I should note that this library is in no way endorsed by the Gtmpl team. However I hope you find it helpful.

I wanted to keep the demo function simple, but ```q_render``` works with anything that impls
Into for Value And these can of course be Gtmpl Derived like most things.
However that is tricky to demo in a doctest.

If you wish to add more helpers send me a PR and I'll be happy to add them, as long as your happy with the MIT licence for everything you contribute.



# changelog :

## v0.1.3 
* add idiv and mod for integer behaviours
* changed with_all to with_defaults, to make it clear that it does not include exec,
* Added exec functionality and a security notice on that
* Added first selector

## v0.1.2:
Now has "sub, div and has" methods also "wrap" added, for performing linewraps on string.
