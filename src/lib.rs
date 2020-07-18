#![recursion_limit = "10240"]

use web_sys::window;

mod css_import;
mod id_generator;
pub mod layout;
pub mod listing;
pub mod widgets;

pub fn init() {
    // language=CSS
    let root_css = "
:root {
    font-size: 16px;

    /* Colors: */
    --white: #FFFFFF;
    --secondary-color: #966554;
    --negative-color: #A61C13;
    --positive-color: #146621;
    --primary-color: #514B57;
    --information-color: #182B70;
    --disabled-border-color: #A8A1AE;
    --disabled-color: #D3D0D7;
    --input-background-color: #F4F3F5;
    --dropback: #DBDBDB;
    --menu-bar-box-shadown: #00000029;

    /* Font/text values */
    --font-family: objektiv-mk1, sans-serif;
    --font-style-regular: 400;
    --font-style-bold: 700;
    --font-style-light: 300;
    --font-size-12: 12px;
    --font-size-16: 16px;
    --font-size-24: 24px;
    --font-size-32: 32px;

    --line-height-18: 18px;
    --line-height-23: 23px;
    --line-height-34: 34px;
    --line-height-46: 46px;

    --background-image-select-primary: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23FFFFFF; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23514b57; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");
    --background-image-select-negative: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23FFFFFF; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23A61C13; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");
    --background-image-select-positive: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23FFFFFF; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23146621; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");
    --background-image-select-disabled: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23FFFFFF; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23A8A1AE; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");

    font-family: var(--font-family);
    font-weight: var(--font-style-regular);
    line-height: var(--line-height-23);
}

html,
body {
    padding: 0;
    margin: 0;
    color: var(--primary-color);
    background: var(--white);
}

@media (prefers-color-scheme: dark) {
    :root {
        --white: #000000;
        --menu-bar-box-shadown: #FFFFFF29;
        --input-background-color: #0B0A0C;
        --dropback: #353535;

        --background-image-select-primary: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23000000; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23514b57; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");
        --background-image-select-negative: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23000000; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23A61C13; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");
        --background-image-select-positive: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23000000; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23146621; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");
        --background-image-select-disabled: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' width='32' height='16' viewBox='0 0 32 16'%3E%3Cdefs%3E%3Cstyle%3E .cls-1 %7B clip-path: url(%23clip); %7D .cls-2, .cls-5 %7B fill: %23000000; %7D .cls-3, .cls-4 %7B stroke: none; %7D .cls-4 %7B fill: %23A8A1AE; %7D %3C/style%3E%3CclipPath id='clip'%3E%3Crect width='32' height='16'/%3E%3C/clipPath%3E%3C/defs%3E%3Cg class='cls-1'%3E%3Crect class='cls-5' width='32' height='16'/%3E%3Cg class='cls-2' transform='translate(32 16) rotate(180)'%3E%3Cpath class='cls-3' d='M 27.17156982421875 15.00000286102295 L 4.828429222106934 15.00000286102295 C 4.208438873291016 15.00000286102295 3.964539051055908 14.52750301361084 3.904549121856689 14.38268280029297 C 3.844568967819214 14.23787307739258 3.682919025421143 13.73129272460938 4.121328830718994 13.29289245605469 L 15.2928991317749 2.121322631835938 C 15.48176860809326 1.932442665100098 15.73288917541504 1.828422665596008 15.99999904632568 1.828422665596008 C 16.26710891723633 1.828422665596008 16.51822853088379 1.932442665100098 16.70709991455078 2.121322631835938 L 27.8786792755127 13.29289245605469 C 28.31707954406738 13.73129272460938 28.15543937683105 14.2378625869751 28.09544944763184 14.38268280029297 C 28.03546905517578 14.52750301361084 27.79156875610352 15.00000286102295 27.17156982421875 15.00000286102295 Z'/%3E%3Cpath class='cls-4' d='M 15.99999904632568 2.828422546386719 L 4.828422546386719 13.99998950958252 C 4.828424453735352 13.9999942779541 4.828426361083984 13.99999809265137 4.828428268432617 14.00000286102295 L 27.17156219482422 14.00000286102295 C 27.17156410217285 13.99999618530273 27.17156600952148 13.99998950958252 27.17157936096191 13.99999237060547 L 16.00190925598145 2.828592300415039 C 16.0017204284668 2.828542709350586 16.00099945068359 2.828422546386719 15.99999904632568 2.828422546386719 M 15.99999618530273 0.828425407409668 C 16.51183891296387 0.828425407409668 17.02368545532227 1.023687362670898 17.41420936584473 1.414212226867676 L 28.58578872680664 12.58578300476074 C 29.8457088470459 13.84571266174316 28.95337867736816 16.00000190734863 27.17156982421875 16.00000190734863 L 4.828428268432617 16.00000190734863 C 3.046619415283203 16.00000190734863 2.154279708862305 13.84571266174316 3.414209365844727 12.58578300476074 L 14.58578872680664 1.414212226867676 C 14.97630882263184 1.023687362670898 15.48815155029297 0.828425407409668 15.99999618530273 0.828425407409668 Z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E\");
    }
}

h1, h2, h3, h4, h5, h6 {
    text-decoration: underline;
    margin-bottom: 0;
}

h1 {
    font-size: var(--font-size-32);
    line-height: var(--line-height-46);
}

h2 {
    font-size: var(--font-size-24);
    line-height: var(--line-height-34);
}

h3 {
    font-size: var(--font-size-16);
    line-height: var(--line-height-23);
}
";
    let css = vec![
        root_css,
        layout::button_row::get_css(),
        layout::form::get_css(),
        layout::page::get_css(),
        layout::row::get_css(),
        listing::card::card::get_css(),
        listing::card::card_button::get_css(),
        listing::card::card_button_row::get_css(),
        listing::card::card_header::get_css(),
        listing::card::card_container::get_css(),
        listing::table::get_css(),
        widgets::alert::get_css(),
        widgets::button::get_css(),
        widgets::dialog::utils::dialog::get_css(),
        widgets::floating_action_button::get_css(),
        widgets::form::input::get_css(),
        widgets::form::checkbox::get_css(),
        widgets::form::radio::get_css(),
        widgets::form::dropdown::get_css(),
        widgets::form::multi_select::get_css(),
        widgets::form::file_upload::get_css(),
        widgets::menu::bar::get_css(),
        widgets::menu::item::get_css(),
        widgets::toast::get_css(),
    ];
    let doc = window().unwrap().document().unwrap();

    let head = doc
        .query_selector("head")
        .expect("Head not found")
        .expect("Head not found");
    let style_element = doc.create_element("style").unwrap();
    style_element.set_text_content(Some(&css.join("\n")));
    head.append_child(&style_element)
        .expect("Could not insert style tag");
    head.append_child(&css_import::get_mdi(&doc))
        .expect("Could not insert link tag");
    head.append_child(&css_import::get_typekit(&doc))
        .expect("Could not insert link tag");

    let body = doc
        .query_selector("body")
        .expect("Body not found")
        .expect("Body not found");
    body.append_child(&widgets::toast::toast_container())
        .expect("Body not found");
}
