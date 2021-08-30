(defstruct (word-filter (:conc-name wf-))
  (pre-table (make-hash-table :test 'equal) :type hash-table)
  (pro-table (make-hash-table :test 'equal) :type hash-table))

(defun new-word-filter (words)
  ;;(declare (list string words))
  (let ((wf (make-word-filter))
        (w-words (loop
                   with index = 0
                   and table = (make-hash-table :test 'equal)
                   for word of-type string in words
                   do (setf (gethash word table) index)
                   do (incf index)
                   finally (return table))))
    (loop
      for word being each hash-key of w-words
        using (hash-value ind)
      do (loop
           for index from 1 to (length word)
           for prefix = (subseq word 0 index)
           and suffix = (subseq word (- (length word) index) (length word))

           when (not (gethash prefix (wf-pre-table wf)))
             ;; fake hash-set, key is fixnum
             do (setf (gethash prefix (wf-pre-table wf)) (make-hash-table)) 
           end

           when (not (gethash suffix (wf-pro-table wf)))
             ;; fake hash-set, key is fixnum
             do (setf (gethash suffix (wf-pro-table wf)) (make-hash-table)) 
           end

           do (setf (gethash ind (gethash prefix (wf-pre-table wf))) 0
                    (gethash ind (gethash suffix (wf-pro-table wf))) 0)
           ))
    wf))

(defun return-all-keys (table)
  (loop for k being each hash-key of table
        collect k))

(defun f (wf preffix suffix)
  (let ((k1 (return-all-keys (gethash preffix (wf-pre-table wf))))
        (k2 (return-all-keys (gethash suffix (wf-pro-table wf)))))
    (let ((result (intersection k1 k2)))
      (if (not result)
          -1
          (apply #'max result)))
    ))

(defun main ()
  (let ((wf (new-word-filter '("cabaabaaaa" "ccbcababac" "bacaabccba" "bcbbcbacaa" "abcaccbcaa" "accabaccaa" "cabcbbbcca" "ababccabcb" "caccbbcbab" "bccbacbcba"))))
    (assert (= 5 (f wf "a" "aa")))
    (assert (= 9 (f wf "bccbacbcba" "a")))))
