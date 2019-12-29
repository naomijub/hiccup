extern crate hiccup;

use hiccup::hiccup;

#[test]
fn xml_templating() {
    let mut out = String::new();

    let _ = hiccup!(&mut out,
        xml{metas=>"I forgot them all", version=>"any version"}[
            family[name{name=>"Rubiechiov", origin=>"Kazakhstan"}]
            members{class=>"close family"}[
                member{age=>"oldest", color=>"yellow"}["some name"]
                member{age=>"mid-age", color=>"yellow"}["some other name"]
                member{age=>"yougest", color=>"brown"}["Julia"]]
        ]);

    assert_eq!(out,"<xml metas=\"I forgot them all\" version=\"any version\"><family>\
    <name name=\"Rubiechiov\" origin=\"Kazakhstan\"/></family><members class=\"close family\">\
    <member age=\"oldest\" color=\"yellow\">some name</member>\
    <member age=\"mid-age\" color=\"yellow\">some other name</member>\
    <member age=\"yougest\" color=\"brown\">Julia</member></members></xml>");
}
