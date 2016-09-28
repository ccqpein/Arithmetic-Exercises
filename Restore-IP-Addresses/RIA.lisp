(defvar *test1 "25525511135")
(defvar *test2 "0000")
(defvar *test3 "010010")
(defvar *test4 "101023")

(defun cut-ip (str)
  (loop
     for i from 0 to (- (min (length str) 4) 2)
     collect (list (subseq str 0 (1+ i)) (subseq str (1+ i)))
       ))

(defun next-cut (strL)
  (cond ((= (length strL) 1) 
         (cut-ip (car strL)))
        ((= (length (car (last strL))) 1)
         (list strL))
        (t
         (loop for i in (cut-ip (car (last strL))) collect
              (append (butlast strL) i)))))

(defun all-less-255? (l)
  (loop
     for i in l do
       (if (> (parse-integer i) 255)
           (return-from all-less-255? nil)))
  t)

(defun tell-zero-head? (l)
  (loop
     for i in l do
       (if (and (> (length i) 1)
                (char= (elt i 0) #\0))
           (return-from tell-zero-head? nil)))
  t)

(defun merge-ip (l)
  (let ((result (car l))
        (leftL (cdr l)))
    (dolist (this leftL result)
      (setf result
            (concatenate 'string result "." this)))))

(defun restore-ip-address (str)
  (let ((reStr (list (list str))))
    (loop
       for i from 1 to 3
       for temp = '()
       do (loop
             for l in reStr
             do (setf temp (append temp (next-cut l))))
       do (setf reStr temp))
    (setf reStr
          (loop
             for l in reStr
             when (and (= (length l) 4)
                       (all-less-255? l)
                       (tell-zero-head? l))
             collect (merge-ip l)))
    reStr))
