%{
  configs: [
    %{
      name: "default",
      checks: [
        {Credo.Check.Readability.ModuleDoc, false}
      ]
    }
  ]
}
