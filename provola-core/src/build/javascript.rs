pub(crate) fn build(source: &crate::Source) -> Result<crate::Executable, crate::Error> {
    super::interpret(source, "bash")
}
