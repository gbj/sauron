- [X] Paragraph typing animation should support all other element nodes
    - algorithm:
        - for each node that contains text, increment the text length based on the number of characters in the text content
        - if the node does not contain text, increment by one.
            - The animation for element node is appending that node to the parent.
- [X] centralize the color in a root css
    - Added Theme module
- [X] add support for header bar
- [X] take inspiration from https://robertsspaceindustries.com/starmap/ for some other controls
    - button with slanted highlights to the sides and bottom
- [X] Convert the string styles into jss
- [X] Add more style to button such as showing hightlight bar in the bottom, top, left, or right
    - [X] Add a hover highlight at the bottom
- [X] Support for scoped style
    - This is done by prepending namespace to the selectors and class names used in the component
- [ ] Fix issued with Paragraph throwing a runtime error and panics.
- [X] Fix issue with webkit not displaying the animation list
    - webkit does not affect opacity:0 to span
- [ ] Make the futuristic button be in array with permutation of fui_button flag features
- [X] Make an image component by using the Frame which wraps the img element

