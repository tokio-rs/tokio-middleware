(function() {var implementors = {};
implementors["tokio_middleware"] = ["impl&lt;S&gt; Service for <a class=\"struct\" href=\"tokio_middleware/struct.Log.html\" title=\"struct tokio_middleware::Log\">Log</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Service,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::Request: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>","impl&lt;S, E&gt; Service for <a class=\"struct\" href=\"tokio_middleware/struct.Timeout.html\" title=\"struct tokio_middleware::Timeout\">Timeout</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Service&lt;Error = E&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;TimeoutError&lt;S::Future&gt;&gt;,&nbsp;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
