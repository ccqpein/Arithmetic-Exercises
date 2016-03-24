(load "/Users/ccQ/quicklisp/setup.lisp")
(ql:quickload "drakma")
(ql:quickload "chanl")

(defmacro time-it (&body body)
  (let ((start-time (gensym)))
    `(let ((,start-time (get-internal-real-time)))
       ,@body
       (format t "Runtime: ~a milliseconds.~%" (- (get-internal-real-time) ,start-time)))))

(defvar *url*
  (list
   "http://blog.thezerobit.com/"
   "http://quicklisp.org/"
   "http://www.cliki.net/index"
   "http://sbcl.org/"
   ))

(defun do-request (url)
  (let ((start-time (get-internal-real-time)))
    (format t "Starting request: ~a~%" url)
    (drakma:http-request url)
    (let ((elapsed (- (get-internal-real-time) start-time)))
      (format t "Completed request in ~a ms: ~a~%" elapsed url))))

;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;

(defun do-request-chan (url chan)
  (let ((start-time (get-internal-real-time)))
    (chanl:send chan (format nil "Starting request: ~a~%" url))
    (drakma:http-request url)
    (let ((elapsed (- (get-internal-real-time) start-time)))
      (chanl:send chan
                  (format nil "Completed request in ~a ms: ~a~%" elapsed url)))))

#|
(time-it (dolist (url *urls*)
           (do-request url)))

(time-it
  (let ((chan (make-instance 'chanl:channel)))
    (dolist (url *urls*)
      (chanl:pexec () (do-request-chan url chan)))
    (dotimes (x 8)
      (format t (chanl:recv chan)))))
|#

