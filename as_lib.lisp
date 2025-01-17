#!/usr/bin/env -S sbcl --script
(define-alien-callable fuckoff sb-alien:int ()
 		       (format t "Fuck off now")
 		       2)

(format t "What's it doing?")

(save-lisp-and-die "lisp.lib"
		   :callable-exports '("fuckoff"))
