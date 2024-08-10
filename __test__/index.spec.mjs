import test from 'ava'

import { Converter } from '../index.js'

const converter = new Converter();

test('*m to mile', (t) => {
    t.is(converter.convert("1Mm to mile"), "621.37 mile");
    t.is(converter.convert("10km to mile"), "6.21 mile");
    t.is(converter.convert("200m to mile"), "0.12 mile");
});

test('*m to inch', (t) => {
    t.is(converter.convert("45mm to inch"), "1.77 inch");
    t.is(converter.convert("10cm to inch"), "3.94 inch");
});

test('*m to *m', (t) => {
    t.is(converter.convert("45mm to cm"), "4.5 cm");
    t.is(converter.convert("10cm to m"), "0.1 m");
    t.is(converter.convert("1200m to km"), "1.2 km");
    t.is(converter.convert("1234km to Mm"), "1.23 Mm");
});

test('temperatures', (t) => {
    t.is(converter.convert("25C to F"), "77 F");
    t.is(converter.convert("77F to C"), "25 C");

    t.is(converter.convert("25C to K"), "298.15 K");
    t.is(converter.convert("298K to C"), "24.85 C");

    t.is(converter.convert("76F to K"), "297.59 K");
    t.is(converter.convert("298K to F"), "76.73 F");
});
