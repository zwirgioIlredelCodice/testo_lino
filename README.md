# welcome to testolino
a tool to convert text to html pages similar to markdown

## to compile 
`rustc main.rs -o testolino`

## to run 
`./testolino inputFile outputFile`

## syntax
`.!`
# for headers

`.#text#.` **for bold**
`.$text$.` *for italic*

`.*`
* unordered
* lists

`.0`
1. for ordered
1. lists

`.|# header1 |# header2`
`.| cell1 | cell2`

First header | Second header
------------ | -------------
cell 1 | cell 2
