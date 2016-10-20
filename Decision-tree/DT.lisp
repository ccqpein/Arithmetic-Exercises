(defclass node ()
  ((tag
    :accessor tag
    :initarg :this-node)
   (playList
    :accessor playl
    :initarg :this-play)))

(defclass DTree ()
  ((root
    :accessor root
    :initarg :root
    :documentation "this is the node of decision tree")
   (children
    :accessor leaves
    :initarg :leaves
    :documentation "this is the list of children node of this "))
  (:documentation "Decision tree"))


(defvar root (make-instance 'node :this-node "root" :this-play '(9 5)))
(defvar overcast (make-instance 'node :this-node "over" :this-play '(4 0)))
(defvar sunny (make-instance 'node :this-node "sun" :this-play '(2 3)))
(defvar rain (make-instance 'node :this-node "rain" :this-play '(3 2)))
(defvar windy (make-instance 'node :this-node "windy" :this-play '(0 2)))
(defvar no-windy (make-instance 'node :this-node "no-windy" :this-play '(3 0)))
(defvar hum>70 (make-instance 'node :this-node "hum > 70" :this-play '(0 3)))
(defvar hum<70 (make-instance 'node :this-node "hum < 70" :this-play '(2 0)))

