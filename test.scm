(def load-file
  (λ [filename]
     (eval (read-str
            (str-append "(do "
                        (slurp filename)
                        ")")))))

(def hi-world
  (λ [name]
     (println (str-append "hello " name))))

(hi-world "from script")

(if nil
    (println "nil == true")
    "return value")
