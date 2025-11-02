# i1401-charconv

The IBM 1401 uses characters that are not present in ASCII. Different emulators have different ways of handling such special characters. **i1401-charconv** converts between these encodings.

## Usage

`i1401-charconv --from encoding --to encoding < source.txt > destination.txt`

Depending on the emulator, it may also be necessary to convert with iconv afterwards. `i1401-charconv` only handles UTF-8.

Note that newlines are not preserved, and may depend on the OS `i1401-charconv` is run on. An extra newline will be present at the end.

## Supported encodings

* `refeernce`, an encoding that displays each character as shown in the [IBM 1401 reference from April 1962](https://bitsavers.org/pdf/ibm/1401/A24-1403-5_1401_Reference_Apr62.pdf). No known emulator uses this.
* `vdc`, the encoding used by [Michael Schuetz's Virtual Data Center](https://rolffson.de/), a 3D virtual datacenter with an IBM 1401.
* `simh-new-a`, `simh-new-h`, and `simh-old`, the encodings supported by [SIMH](https://simh.trailing-edge.com/). Note that many files on https://ibm-1401.info appear to use `simh-old`
* `sim1401-1047`, the encoding used by IBM's 1401 emulator for the System/360. As this is inherently BCD <-> EBCDIC, `i1401-charconv` then converts that EBCDIC assuming that it's EBCDIC-1047. For use with Hercules, it is suggested to run `codepage 819/1047` at the console.
* `newcomer`, the encoding used by [Joe Newcomer's IBM 1401 emulator for DOS](https://www.piercefuller.com/oldibm-shadow/1401.html). Note that this emulator uses and assumes CP-437, rather than Unicode or ASCII. Example suggested usage, with iconv: `i1401-charconv -f simh-old -t newcomer < bigprint2013.txt | iconv -f utf-8 -t cp437 > bpdos.txt`. The following has been tested and works to transfer Newcomer's sample deck to VDC: `iconv -f cp437 -t utf-8 < SAMPLE.CDR | i1401-charconv.exe -f newcomer -t vdc > sample-vdc.txt`

## Table

A table illustrating each encoding is available at https://sgeo.github.io/i1401-charconv/encodings.htm

## Todo

* `unicode-print-a` and `unicode-print-h` encodings: The IBM 1401 doesn't necessarily print the same characters it receives.
* Todo: Validate `newcomer` functionality