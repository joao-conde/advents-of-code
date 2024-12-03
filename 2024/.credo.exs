%{
  configs: [
    %{
      name: "default",
      checks: [
        {Credo.Check.Readability.ModuleDoc, false},
        {Credo.Check.Refactor.Nesting, [max_nesting: 3]}
      ]
    }
  ]
}
