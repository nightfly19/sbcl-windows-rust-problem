#!/usr/bin/env -S sbcl --script
(define-alien-callable moocow sb-alien:int ()
 		       (format t "Moooo! ~%")
		       (finish-output)
 		       137)

(save-lisp-and-die "lisp.lib"
		   :callable-exports '(
				       "moocow"
				       ))
