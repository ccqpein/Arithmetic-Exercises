(defclass user ()
  ((name :initarg :name)
   (id :initarg :id)
   (age :initarg :age)))

(defmethod name ((a user))
  (slot-value a 'name ))
(defmethod id ((a user))
  (slot-value a 'id))
(defmethod age ((a user))
  (slot-value a 'age))

(defvar *user-table* (make-hash-table))
(setf (gethash 0 *user-table*) (make-instance 'user :name "exciting" :age 93))

(defmacro human (id methods)
  `(let ((a (gethash ,id *user-table*)))
     (list ,@(loop for m in methods 
                collect `(,m a)))))

(defun query (statement)
  (eval (read-from-string statement)))

(query "(human 0 (name age))")

