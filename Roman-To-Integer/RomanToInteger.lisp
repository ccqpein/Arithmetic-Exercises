(defun switch-Number (s)
  (do* ((sumNumbers 0 (+ sumNumbers char2))
	(listOfNumbers (coerce s 'list) (cdr listOfNumbers))
	(char1 0 char2)
	(char2 (char-to-number (car listOfNumbers) char1)
	       (char-to-number (car listOfNumbers) char1)))
       ((equalp listOfNumbers nil)
	(return-from switch-Number sumNumbers))))

(defun char-to-number (char-in-list char1)
  (cond ((equal char-in-list #\I) (return-from char-to-number
				    (cut-number 1 char1)))
	((equal char-in-list #\V) (return-from char-to-number
				    (cut-number 5 char1)))
	((equal char-in-list #\X) (return-from char-to-number
				    (cut-number 10 char1)))
	((equal char-in-list #\D) (return-from char-to-number
				    (cut-number 500 char1)))
	((equal char-in-list #\L) (return-from char-to-number
				    (cut-number 50 char1)))
	((equal char-in-list #\C) (return-from char-to-number
				    (cut-number 100 char1)))
	((equal char-in-list #\M) (return-from char-to-number
				    (cut-number 1000 char1)))
	))

(defun cut-number (n char1)
  (cond ((and (= n 5) (= char1 1)) (return-from cut-number (- n 2)))
	((and (= n 10) (= char1 1)) (return-from cut-number (- n 2)))
	((and (= n 1000) (= char1 100)) (return-from cut-number (- n 200)))
	((and (= n 500) (= char1 100)) (return-from cut-number (- n 200)))
	((and (= n 100) (= char1 10)) (return-from cut-number (- n 20)))
	((and (= n 50) (= char1 10)) (return-from cut-number (- n 20)))
	(t (return-from cut-number n))
	))

