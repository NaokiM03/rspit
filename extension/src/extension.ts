import * as vscode from "vscode";

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

    if (lines.length === 0) {
      return;
    }

    const targetLines: {
      content: string;
      number: number;
    }[] = lines.filter((line) => line.content.match(/fn main\(\)+\s/) !== null);

    const ranges: vscode.Range[] = targetLines.map(
      (line) =>
        new vscode.Range(
          new vscode.Position(line.number, 0),
          new vscode.Position(line.number, line.content.length)
        )
    );

    return ranges.map(
      (range) =>
        new vscode.CodeLens(range, {
          title: "▶ Run",
          command: "rspit.helloWorld",
          tooltip: "Run this snippet",
        })
    );
  }
}

export function activate(context: vscode.ExtensionContext) {
  let disposable = vscode.commands.registerCommand("rspit.helloWorld", () => {
    vscode.window.showInformationMessage("Hello World from RSPIT!");
  });

  const codeLensProvider = new CodeLensProvider();
  const codeLensProviderDisposable = vscode.languages.registerCodeLensProvider(
    { language: "rust" },
    codeLensProvider
  );

  context.subscriptions.push(disposable);
  context.subscriptions.push(codeLensProviderDisposable);
}

export function deactivate() {}
