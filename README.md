# Image Viewer
I have one big problem with my current installation: there's no image viewer that I like.
I would like feh, but it has too many command line options, and I figured it'd be a good exercise to make one myself.
As of now, it supports opening a directory, and will load images ending in ".png", ".jpg", and ".jpeg", although it is a goal to be more dynamic in the future.
It currently allows for sorting randomly and alphabetically.

# current goals
- nice enough looking ui
- click parts for next
    - previous might be hard
- multiple sorting options
    - date modified?  might be annoying
- multithreaded buffering (really only if its needed)
- wraparound buffering? dunno if it's properly in there
- size maximums, dynamic?
- sort into individual files
- zooming?
- name updates with name of file
    - doesn't seem very possible, hover?
- clap
- a help popup

# stretch goals
- handling images without proper suffixes? Through load errors?
- config file
- handle ALL errors
- open on a single image
- handle opening with no images
- get images from image crate so it loads more?

- pdf loader??
    - this might be a fully seperate project
