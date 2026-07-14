import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import App from "./App";

describe("App", () => {
  it("renders MCP Visualizer heading", () => {
    render(<App />);
    expect(screen.getByText("MCP Visualizer")).toBeDefined();
  });

  it("renders description text", () => {
    render(<App />);
    expect(
      screen.getByText("Manage and monitor MCP servers across your AI coding tools.")
    ).toBeDefined();
  });
});
