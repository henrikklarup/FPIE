# Includefile syntax
Files and folders are defined using paths relative to the context path.
You can include files by path:

    includefile
Same goes for folders, including all files, and subfolders (all listed options produce same result):

    - includedir/*
    - includedir/
    - includedir
Prepend a path with `!` for exclusion:

    !excludefile
Same goes for an entire directory:

    !excludedir
It is possible to exclude using glob patterns:

    - !**/*.pyc
    - **/*.tst
    - *.a
    - **/foldername
Comments need to be on a separate line:
    #This is a comment
    includefile
    excludefile
