(function() {var type_impls = {
"consensus_config":[],
"mysten_util_mem":[],
"sui_core":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RandPrime%3Cu64%3E-for-R\" class=\"impl\"><a href=\"#impl-RandPrime%3Cu64%3E-for-R\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; RandPrime&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u64.html\">u64</a>&gt; for R<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand/rng/trait.Rng.html\" title=\"trait rand::rng::Rng\">Rng</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime\" class=\"method trait-impl\"><a href=\"#method.gen_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.usize.html\">usize</a>, _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime within the given bit size limit <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_prime_exact</a>(\n    &amp;mut self,\n    bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.usize.html\">usize</a>,\n    _: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;PrimalityTestConfig&gt;,\n) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random prime with <strong>exact</strong> the given bit size <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime within the given bit size limit. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.gen_safe_prime_exact\" class=\"method trait-impl\"><a href=\"#method.gen_safe_prime_exact\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gen_safe_prime_exact</a>(&amp;mut self, bit_size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.usize.html\">usize</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Generate a random (Sophie German) safe prime with the <strong>exact</strong> given bit size. The generated prime\nis guaranteed to pass the [is_safe_prime][crate::nt_funcs::is_safe_prime] test <a>Read more</a></div></details></div></details>","RandPrime<u64>","sui_core::checkpoints::CheckpointHeight","sui_core::consensus_throughput_calculator::TimestampSecs"]],
"sui_rosetta":[],
"sui_types":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CSequenceNumber%3E-for-u64\" class=\"impl\"><a class=\"src rightside\" href=\"src/sui_types/base_types.rs.html#1165-1169\">source</a><a href=\"#impl-From%3CSequenceNumber%3E-for-u64\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"sui_types/base_types/struct.SequenceNumber.html\" title=\"struct sui_types::base_types::SequenceNumber\">SequenceNumber</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u64.html\">u64</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sui_types/base_types.rs.html#1166-1168\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(val: <a class=\"struct\" href=\"sui_types/base_types/struct.SequenceNumber.html\" title=\"struct sui_types::base_types::SequenceNumber\">SequenceNumber</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<SequenceNumber>","sui_types::base_types::TxSequenceNumber","sui_types::committee::EpochId","sui_types::committee::StakeUnit","sui_types::messages_checkpoint::CheckpointSequenceNumber","sui_types::messages_checkpoint::CheckpointTimestamp","sui_types::messages_consensus::Round","sui_types::messages_consensus::TimestampMs"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MoveTypeTagTrait-for-u64\" class=\"impl\"><a class=\"src rightside\" href=\"src/sui_types/lib.rs.html#211-215\">source</a><a href=\"#impl-MoveTypeTagTrait-for-u64\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"sui_types/trait.MoveTypeTagTrait.html\" title=\"trait sui_types::MoveTypeTagTrait\">MoveTypeTagTrait</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u64.html\">u64</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.get_type_tag\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sui_types/lib.rs.html#212-214\">source</a><a href=\"#method.get_type_tag\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sui_types/trait.MoveTypeTagTrait.html#tymethod.get_type_tag\" class=\"fn\">get_type_tag</a>() -&gt; <a class=\"enum\" href=\"sui_types/enum.TypeTag.html\" title=\"enum sui_types::TypeTag\">TypeTag</a></h4></section></div></details>","MoveTypeTagTrait","sui_types::base_types::TxSequenceNumber","sui_types::committee::EpochId","sui_types::committee::StakeUnit","sui_types::messages_checkpoint::CheckpointSequenceNumber","sui_types::messages_checkpoint::CheckpointTimestamp","sui_types::messages_consensus::Round","sui_types::messages_consensus::TimestampMs"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()