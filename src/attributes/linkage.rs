use std::fmt::{Display, Formatter};

#[derive(Debug,Eq,PartialEq)]
pub enum Linkage {
    /// [Global] values with [Private](Linkage::Private) linkage are only directly accessible by objects in the current [Module].
    /// In particular, linking code into a [Module] with a private global value may cause the private to be renamed as necessary to avoid collisions.
    /// Because the symbol is private to the [Module], all references can be updated.
    /// This doesn't show up in any symbol table in the object file.
    ///
    /// [Module]: crate::module_elements::module::Module
    /// [Global]: crate::module_elements::Global
    Private,

    /// Similar to [Private](Linkage::Private), but the value shows as a local symbol (STB_LOCAL in the case of ELF) in the object file.
    /// This corresponds to the notion of the ‘static’ keyword in C.
    Internal,

    /// [Globals][Global] with
    /// [AvailableExternally](Linkage::AvailableExternally) linkage are never emitted into the
    /// object file corresponding to the LLVM [Module].
    ///
    /// From the linker’s perspective, an [AvailableExternally](Linkage::AvailableExternally)
    /// [Global] is equivalent to an external declaration. They exist to allow inlining and other
    /// optimizations to take place given knowledge of the definition of the [Global], which is
    /// known to be somewhere outside the [module][Module].
    ///
    /// [Globals][Global] with
    /// [AvailableExternally](Linkage::AvailableExternally) linkage are allowed to be discarded at
    /// will, and allow inlining and other optimizations. This linkage type is only allowed on
    /// definitions, not declarations.
    ///
    /// [Module]: crate::module_elements::module::Module
    /// [Global]: crate::module_elements::Global
    AvailableExternally,

    /// [Globals][Global] with [LinkOnce](Linkage::LinkOnce) linkage are merged with other
    /// [globals][Global] of the same name when linkage occurs. This can be used to implement some
    /// forms of inline functions, templates, or other code which must be generated in each
    /// translation unit that uses it, but where the body may be overridden with a more definitive
    /// definition later. Unreferenced [LinkOnce](Linkage::LinkOnce) [globals][Global] are allowed
    /// to be discarded.
    ///
    /// Note that [LinkOnce](Linkage::LinkOnce) linkage does not actually allow the optimizer to
    /// inline the body of this function into callers because it doesn't know if this definition of
    /// the function is the definitive definition within the program or whether it will be
    /// overridden by a stronger definition. To enable inlining and other optimizations, use
    /// [LinkOnceODR](Linkage::LinkOnceODR) linkage.
    ///
    /// [Global]: crate::module_elements::Global
    LinkOnce,

    /// ``Weak`` linkage has the same merging semantics as ``LinkOnce`` linkage, except that unreferenced ``Global``(s) with ``Weak`` linkage may not be discarded.
    ///  This is used for globals that are declared “weak” in C source code.
    Weak,

    /// ``Common`` linkage is most similar to ``Weak`` linkage, but they are used for tentative definitions in C, such as “int X;” at global scope. Symbols with ``Common`` linkage are merged in the same way as ``Weak`` symbols, and they may not be deleted if unreferenced. `Common` symbols may not have an explicit section, must have a zero initializer, and may not be marked ‘constant’. Functions and aliases may not have ``Common`` linkage.
    Common,

    /// ``Appending`` linkage may only be applied to ``Global`` variables of pointer to array type. When two global variables with ``Appending`` linkage are linked together, the two global arrays are appended together. This is the LLVM, typesafe, equivalent of having the system linker append together “sections” with identical names when .o files are linked.
    /// Unfortunately this doesn't correspond to any feature in .o files, so it can only be used for variables like `llvm.global_ctors` which llvm interprets specially.
    Appending,

    /// The semantics of this linkage follow the ELF object file model: the symbol is weak until linked, if not linked, the symbol becomes null instead of being an undefined reference.
    ExternWeak,

    /// Some languages allow differing ``Global``(s) to be merged, such as two functions with different semantics. Other languages, such as C++, ensure that only equivalent globals are ever merged (the “one definition rule” — “ODR”). Such languages can use the ``LinkOnceODR`` and ``WeakODR`` linkage types to indicate that the ``Global`` will only be merged with equivalent globals. These linkage types are otherwise the same as their non-odr versions.
    LinkOnceODR,

    /// Some languages allow differing ``Global``(s) to be merged, such as two functions with different semantics. Other languages, such as C++, ensure that only equivalent globals are ever merged (the “one definition rule” — “ODR”). Such languages can use the ``LinkOnceODR`` and ``WeakODR`` linkage types to indicate that the ``Global`` will only be merged with equivalent globals. These linkage types are otherwise the same as their non-odr versions.
    WeakODR,

    /// If none of the above identifiers are used, the ``Global`` is externally visible, meaning that it participates in linkage and can be used to resolve external symbol references.
    External,
}

impl Display for Linkage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str( match self {
            Linkage::Private => "private",
            Linkage::Internal => "internal",
            Linkage::AvailableExternally => "available_externally",
            Linkage::LinkOnce => "link_once",
            Linkage::Weak => "weak",
            Linkage::Common => "common",
            Linkage::Appending => "appending",
            Linkage::ExternWeak => "extern_weak",
            Linkage::LinkOnceODR => "link_once_odr",
            Linkage::WeakODR => "weak_odr",
            Linkage::External => "external",
        })
    }
}

#[cfg(test)]
pub mod linkage_tests {
    use crate::attributes::linkage::Linkage::*;

    #[test]
    fn test_stringify() {
        assert_eq!(format!("{}", LinkOnce), "link_once");
        assert_eq!(format!("{}", External), "external");
    }
}