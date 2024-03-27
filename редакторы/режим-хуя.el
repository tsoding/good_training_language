;; СДЕЛАТЬ: расширения для vim
(defconst синтаксическая-таблица-режима-хуя
  (with-syntax-table (copy-syntax-table)
    ;; Комментарии в стиле Си/Си++
	(modify-syntax-entry ?/ ". 124b")
	(modify-syntax-entry ?* ". 23")
	(modify-syntax-entry ?\n "> b")

    ;; Строковые литералы
    (modify-syntax-entry ?' "\"")
    (modify-syntax-entry ?« "(»")
    (modify-syntax-entry ?» ")«")

    (syntax-table))
  "Синтаксическая таблица для `режим-хуя'.")

(eval-and-compile
  (defconst ключевые-слова-хуя
    '("пер" "про" "конст" "пока" "нч" "кц" "для"
      "если" "то" "иначе" "вернуть" "замкнуть" "пропустить"
      "структ" "союз" "как" "вкл" "внешняя" "библ" "или"
      "и" "истина" "ложь" "лбс" "пбс" "ост")))

;; СДЕЛАТЬ: подсветка многострочных «ёлочных» литералов.
;; СДЕЛАТЬ: экранированные «ёлочки» внутри «ёлочных» литералов.
(defconst подсветка-хуя
  `(("«.*?»" . font-lock-string-face)
    (,(regexp-opt ключевые-слова-хуя 'symbols) . font-lock-keyword-face)))

;;;###autoload
(define-derived-mode режим-хуя prog-mode "хуя"
  "Основной режим (анг. Major mode) для редактирования исходных файлов на языке ХУЯ."
  :syntax-table синтаксическая-таблица-режима-хуя
  (setq font-lock-defaults '(подсветка-хуя))
  (setq-local comment-start "// "))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.хуя\\'" . режим-хуя))
