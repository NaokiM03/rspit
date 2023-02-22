import * as vscode from "vscode";

import * as TOML from "@iarna/toml";

type RunSnippetArg = {
  filePath: string;
  package: string;
};

class CodeLensProvider implements vscode.CodeLensProvider {
  constructor() {}

  public provideCodeLenses(
    document: vscode.TextDocument,
    _token: vscode.CancellationToken
  ): vscode.ProviderResult<vscode.CodeLens[]> {
    const fileContent: string = document.getText();

    const lines: {
      content: string;
      number: number;
    }[] = fileContent.split("\n").map((s: string, i: number) => {
      return { content: s, number: i };
    });

    if (lines.length === 0) return;

    const targetLines: {
      content: string;
      number: number;
    }[] = lines.filter(
      (line) => line.content.match(/^fn main\(\)+\s/) !== null
    );

    if (targetLines.length === 0) return;

    const ranges: vscode.Range[] = targetLines.map(
      (line) =>
        new vscode.Range(
          new vscode.Position(line.number, 0),
          new vscode.Position(line.number, line.content.length)
        )
    );

    const snippets: string[] = fileContent.split("//# ---");
    // Programmers should be aware of incorrect syntax
    // that rust-analyzer does not output errors.
    // In the future, may use Diagnostics.
    // if (targetLines.length !== snippets.length) {}

    const lineNumbers = snippets.map((snippet, index, array) =>
      index !== array.length - 1
        ? snippet.split("\n").length - 1
        : snippet.split("\n").length
    );
    for (let i = 0; i < lineNumbers.length; i++) {
      if (i === 0) {
        lineNumbers[i] = lineNumbers[i];
      } else {
        lineNumbers[i] = lineNumbers[i - 1] + lineNumbers[i];
      }
    }

    return ranges.map((range) => {
      let index = lineNumbers.findIndex(
        (lineNumber) => lineNumber > range.start.line + 1
      );
      const snippet = snippets[index];

      let toml: TOML.JsonMap;
      try {
        const t: string = snippet
          .split("\n")
          .filter((line) => line.startsWith("//# "))
          .map((line) => line.substring(4))
          .join("\n");
        toml = TOML.parse(t);
      } catch (e) {
        throw e;
      }

      const packageField = toml.package as TOML.JsonMap;
      const packageName = packageField.name as string;

      const arg: RunSnippetArg = {
        filePath: document.fileName,
        package: packageName,
      };

      return new vscode.CodeLens(range, {
        title: "▶ Run",
        command: "rspit.runSnippet",
        tooltip: `Run ${packageName} package`,
        arguments: [arg],
      });
    });
  }
}

export function activate(context: vscode.ExtensionContext) {
  let runSnippetDisposable = vscode.commands.registerCommand(
    "rspit.runSnippet",
    async (arg: RunSnippetArg) => {
      // Save snippet before run.
      await vscode.commands.executeCommand("workbench.action.files.save");
      // Clear terminal before run.
      await vscode.commands.executeCommand("workbench.action.terminal.clear");

      const task = new vscode.Task(
        { type: "rspit" },
        vscode.TaskScope.Workspace,
        "rspit run",
        "rspit",
        new vscode.ShellExecution(
          `pit run ${arg.filePath} --package ${arg.package}`
        ),
        []
      );
      try {
        vscode.tasks.executeTask(task);
      } catch (e) {
        throw e;
      }
    }
  );

  const codeLensProvider = new CodeLensProvider();
  const codeLensProviderDisposable = vscode.languages.registerCodeLensProvider(
    { language: "rust" },
    codeLensProvider
  );

  context.subscriptions.push(runSnippetDisposable);
  context.subscriptions.push(codeLensProviderDisposable);
}

export function deactivate() {}
