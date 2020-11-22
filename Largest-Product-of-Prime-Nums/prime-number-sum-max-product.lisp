;;;; Give a number N, if N can be sum of several different prime numbers. What's the largest number of those prime numbers product?

;;; hard code prime numbers list
(defparameter *prime-numbers-list* '(2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97 101 103 107 109 113 127 131 137 139 149 151 157 163 167 173 179 181 191 193 197 199 211 223 227 229 233 239 241 251 257 263 269 271 277 281 283 293 307 311 313 317 331 337 347 349 353 359 367 373 379 383 389 397 401 409 419 421 431 433 439 443 449 457 461 463 467 479 487 491 499 503 509 521 523 541 547 557 563 569 571 577 587 593 599 601 607 613 617 619 631 641 643 647 653 659 661 673 677 683 691 701 709 719 727 733 739 743 751 757 761 769 773 787 797 809 811 821 823 827 829 839 853 857 859 863 877 881 883 887 907 911 919 929 937 941 947 953 967 971 977 983 991 997 1009))

;;; table => {k: ((product (prime numbers*))*)}
;;; like {5: ((6 (2 3))}

(defun max-product (n table)
  ;; 3 and 2 are unique, of course I can put them in table directly
  (cond ((= 3 n) (return-from max-product '((3 (3)))))
        ((= 2 n) (return-from max-product '((2 (2)))))
        ((= 1 n) (return-from max-product nil))
        (t (loop
             for i in (prime-list-below n)
             for records = (record-of-this-number (- n i) table)
             if records
               ;;do (format t "this prime-number is ~a" i)
               ;;and do (format t ", this records is ~a~%" records)
               do (let ((record (check-if-pnumber-already-in i records)))
                        (if record
                            (update-table n i record table)))
             finally (return (gethash n table))
             ))))

(defun record-of-this-number (n table)
  "find out the record of this number in table"
  (let ((r (gethash n table)))
    (if r
        r
        (max-product n table))))

(defun update-table (n i record table)
  "update table with n i record of (n - i)"
  (let* ((a (list (* i (car record))
                  (append (cadr record) (list i)))))
    (setf (gethash n table)
          (sort (append (gethash n table '()) (list a)) #'> :key #'car))))

(defun prime-list-below (n)
  "return prime numbers list below n"
  (reverse (subseq *prime-numbers-list*
           0
           (1+ (position-if (lambda (x) (< x n))
                         *prime-numbers-list*
                         :from-end t)))))

(defun check-if-pnumber-already-in (pn records)
  "return the first record (largest) which not included pn in prime numbers"
  (loop
    for r in records
    if (not (member pn (cadr r)))
      do (return r)
    finally (return nil)))

(defun run (n)
  (let ((table (make-hash-table :test 'equal)))
    (max-product n table)
    ;; (sort (loop
    ;;         for k being the hash-keys of table
    ;;         collect k)
    ;;       #'<)
    ))
