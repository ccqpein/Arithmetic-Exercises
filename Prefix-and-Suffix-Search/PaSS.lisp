(defstruct word-filter
  (pre-table (make-hash-table :test 'equal) :type hash-table)
  (pro-table (make-hash-table :test 'equal) :type hash-table))

(defun f (words)
  ;;(declare (list string words))
  (let ((wf (make-word-filter))
        (w-words (loop
                   with index = 0
                   and table = (make-hash-table :test 'equal)
                   for word of-type string in words
                   do (setf (gethash word table) index)
                   do (incf index)
                   finally (return table))))
    w-words))

(defun main ())


