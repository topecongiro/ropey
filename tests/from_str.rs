extern crate ropey;

use ropey::Rope;

#[test]
fn from_str() {
    // Build rope from file contents
    let mut rope = Rope::from_str(TEXT);

    // Verify rope integrity
    rope.assert_integrity();
    rope.assert_invariants();

    // Verify that they match
    assert_eq!(rope, TEXT);
}

const TEXT: &str = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas sit
amet tellus  nec turpis feugiat semper. Nam at nulla laoreet, finibus
eros sit amet, fringilla  mauris. Fusce vestibulum nec ligula efficitur
laoreet. Nunc orci leo, varius eget  ligula vulputate, consequat
eleifend nisi. Cras justo purus, imperdiet a augue  malesuada, convallis
cursus libero. Fusce pretium arcu in elementum laoreet. Duis  mauris
nulla, suscipit at est nec, malesuada pellentesque eros. Quisque semper
porta  malesuada. Nunc hendrerit est ac faucibus mollis. Nam fermentum
id libero sed  egestas. Duis a accumsan sapien. Nam neque diam, congue
non erat et, porta sagittis  turpis. Vivamus vitae mauris sit amet massa
mollis molestie. Morbi scelerisque,  augue id congue imperdiet, felis
lacus euismod dui, vitae facilisis massa dui quis  sapien. Vivamus
hendrerit a urna a lobortis.

Donec ut suscipit risus. Vivamus dictum auctor vehicula. Sed lacinia
ligula sit amet  urna tristique commodo. Sed sapien risus, egestas ac
tempus vel, pellentesque sed  velit. Duis pulvinar blandit suscipit.
Curabitur viverra dignissim est quis ornare.  Nam et lectus purus.
Integer sed augue vehicula, volutpat est vel, convallis justo.
Suspendisse a convallis nibh, pulvinar rutrum nisi. Fusce ultrices
accumsan mauris  vitae ornare. Cras elementum et ante at tincidunt. Sed
luctus scelerisque lobortis.  Sed vel dictum enim. Fusce quis arcu
euismod, iaculis mi id, placerat nulla.  Pellentesque porttitor felis
elementum justo porttitor auctor.

Aliquam finibus metus commodo sem egestas, non mollis odio pretium.
Aenean ex  lectus, rutrum nec laoreet at, posuere sit amet lacus. Nulla
eros augue, vehicula et  molestie accumsan, dictum vel odio. In quis
risus finibus, pellentesque ipsum  blandit, volutpat diam. Etiam
suscipit varius mollis. Proin vel luctus nisi, ac  ornare justo. Integer
porttitor quam magna. Donec vitae metus tempor, ultricies  risus in,
dictum erat. Integer porttitor faucibus vestibulum. Class aptent taciti
sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.
Vestibulum  ante ipsum primis in faucibus orci luctus et ultrices
posuere cubilia Curae; Nam  semper congue ante, a ultricies velit
venenatis vitae. Proin non neque sit amet ex  commodo congue non nec
elit. Nullam vel dignissim ipsum. Duis sed lobortis ante.  Aenean
feugiat rutrum magna ac luctus.

Ut imperdiet non ante sit amet rutrum. Cras vel massa eget nisl gravida
auctor.  Nulla bibendum ut tellus ut rutrum. Quisque malesuada lacinia
felis, vitae semper  elit. Praesent sit amet velit imperdiet, lobortis
nunc at, faucibus tellus. Nullam  porttitor augue mauris, a dapibus
tellus ultricies et. Fusce aliquet nec velit in  mattis. Sed mi ante,
lacinia eget ornare vel, faucibus at metus.

Pellentesque nec viverra metus. Sed aliquet pellentesque scelerisque.
Duis efficitur  erat sit amet dui maximus egestas. Nullam blandit ante
tortor. Suspendisse vitae  consectetur sem, at sollicitudin neque.
Suspendisse sodales faucibus eros vitae  pellentesque. Cras non quam
dictum, pellentesque urna in, ornare erat. Praesent leo  est, aliquet et
euismod non, hendrerit sed urna. Sed convallis porttitor est, vel
aliquet felis cursus ac. Vivamus feugiat eget nisi eu molestie.
Phasellus tincidunt  nisl eget molestie consectetur. Phasellus vitae ex
ut odio sollicitudin vulputate.  Sed et nulla accumsan, eleifend arcu
eget, gravida neque. Donec sit amet tincidunt  eros. Ut in volutpat
ante.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas sit
amet tellus  nec turpis feugiat semper. Nam at nulla laoreet, finibus
eros sit amet, fringilla  mauris. Fusce vestibulum nec ligula efficitur
laoreet. Nunc orci leo, varius eget  ligula vulputate, consequat
eleifend nisi. Cras justo purus, imperdiet a augue  malesuada, convallis
cursus libero. Fusce pretium arcu in elementum laoreet. Duis  mauris
nulla, suscipit at est nec, malesuada pellentesque eros. Quisque semper
porta  malesuada. Nunc hendrerit est ac faucibus mollis. Nam fermentum
id libero sed  egestas. Duis a accumsan sapien. Nam neque diam, congue
non erat et, porta sagittis  turpis. Vivamus vitae mauris sit amet massa
mollis molestie. Morbi scelerisque,  augue id congue imperdiet, felis
lacus euismod dui, vitae facilisis massa dui quis  sapien. Vivamus
hendrerit a urna a lobortis.

Donec ut suscipit risus. Vivamus dictum auctor vehicula. Sed lacinia
ligula sit amet  urna tristique commodo. Sed sapien risus, egestas ac
tempus vel, pellentesque sed  velit. Duis pulvinar blandit suscipit.
Curabitur viverra dignissim est quis ornare.  Nam et lectus purus.
Integer sed augue vehicula, volutpat est vel, convallis justo.
Suspendisse a convallis nibh, pulvinar rutrum nisi. Fusce ultrices
accumsan mauris  vitae ornare. Cras elementum et ante at tincidunt. Sed
luctus scelerisque lobortis.  Sed vel dictum enim. Fusce quis arcu
euismod, iaculis mi id, placerat nulla.  Pellentesque porttitor felis
elementum justo porttitor auctor.

Aliquam finibus metus commodo sem egestas, non mollis odio pretium.
Aenean ex  lectus, rutrum nec laoreet at, posuere sit amet lacus. Nulla
eros augue, vehicula et  molestie accumsan, dictum vel odio. In quis
risus finibus, pellentesque ipsum  blandit, volutpat diam. Etiam
suscipit varius mollis. Proin vel luctus nisi, ac  ornare justo. Integer
porttitor quam magna. Donec vitae metus tempor, ultricies  risus in,
dictum erat. Integer porttitor faucibus vestibulum. Class aptent taciti
sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.
Vestibulum  ante ipsum primis in faucibus orci luctus et ultrices
posuere cubilia Curae; Nam  semper congue ante, a ultricies velit
venenatis vitae. Proin non neque sit amet ex  commodo congue non nec
elit. Nullam vel dignissim ipsum. Duis sed lobortis ante.  Aenean
feugiat rutrum magna ac luctus.

Ut imperdiet non ante sit amet rutrum. Cras vel massa eget nisl gravida
auctor.  Nulla bibendum ut tellus ut rutrum. Quisque malesuada lacinia
felis, vitae semper  elit. Praesent sit amet velit imperdiet, lobortis
nunc at, faucibus tellus. Nullam  porttitor augue mauris, a dapibus
tellus ultricies et. Fusce aliquet nec velit in  mattis. Sed mi ante,
lacinia eget ornare vel, faucibus at metus.

Pellentesque nec viverra metus. Sed aliquet pellentesque scelerisque.
Duis efficitur  erat sit amet dui maximus egestas. Nullam blandit ante
tortor. Suspendisse vitae  consectetur sem, at sollicitudin neque.
Suspendisse sodales faucibus eros vitae  pellentesque. Cras non quam
dictum, pellentesque urna in, ornare erat. Praesent leo  est, aliquet et
euismod non, hendrerit sed urna. Sed convallis porttitor est, vel
aliquet felis cursus ac. Vivamus feugiat eget nisi eu molestie.
Phasellus tincidunt  nisl eget molestie consectetur. Phasellus vitae ex
ut odio sollicitudin vulputate.  Sed et nulla accumsan, eleifend arcu
eget, gravida neque. Donec sit amet tincidunt  eros. Ut in volutpat
ante.
";
