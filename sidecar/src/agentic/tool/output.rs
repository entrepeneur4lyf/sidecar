//! Contains the output of a tool which can be used by any of the callers

use super::{
    code_symbol::{
        correctness::CodeCorrectnessAction,
        find_file_for_new_symbol::FindFileForSymbolResponse,
        find_symbols_to_edit_in_context::FindSymbolsToEditInContextResponse,
        followup::ClassSymbolFollowupResponse,
        important::CodeSymbolImportantResponse,
        initial_request_follow::CodeSymbolFollowInitialResponse,
        models::anthropic::{
            CodeSymbolShouldAskQuestionsResponse, CodeSymbolToAskQuestionsResponse, ProbeNextSymbol,
        },
        new_sub_symbol::NewSubSymbolRequiredResponse,
        planning_before_code_edit::PlanningBeforeCodeEditResponse,
        probe::ProbeEnoughOrDeeperResponse,
        reranking_symbols_for_editing_context::ReRankingSnippetsForCodeEditingResponse,
    },
    editor::apply::EditorApplyResponse,
    filtering::broker::{
        CodeToEditFilterResponse, CodeToEditSymbolResponse, CodeToProbeFilterResponse,
        CodeToProbeSubSymbolList,
    },
    grep::file::FindInFileResponse,
    lsp::{
        diagnostics::LSPDiagnosticsOutput,
        gotodefintion::GoToDefinitionResponse,
        gotoimplementations::GoToImplementationResponse,
        gotoreferences::GoToReferencesResponse,
        grep_symbol::LSPGrepSymbolInCodebaseResponse,
        open_file::OpenFileResponse,
        quick_fix::{GetQuickFixResponse, LSPQuickFixInvocationResponse},
    },
    rerank::base::ReRankEntriesForBroker,
    swe_bench::test_tool::SWEBenchTestRepsonse,
};

#[derive(Debug)]
pub struct CodeToEditSnippet {
    start_line: i64,
    end_line: i64,
    thinking: String,
}

impl CodeToEditSnippet {
    pub fn start_line(&self) -> i64 {
        self.start_line
    }

    pub fn end_line(&self) -> i64 {
        self.end_line
    }

    pub fn thinking(&self) -> &str {
        &self.thinking
    }
}

#[derive(Debug)]
pub struct CodeToEditToolOutput {
    snipets: Vec<CodeToEditSnippet>,
}

impl CodeToEditToolOutput {
    pub fn new() -> Self {
        CodeToEditToolOutput { snipets: vec![] }
    }

    pub fn add_snippet(&mut self, start_line: i64, end_line: i64, thinking: String) {
        self.snipets.push(CodeToEditSnippet {
            start_line,
            end_line,
            thinking,
        });
    }
}

#[derive(Debug)]
pub enum ToolOutput {
    PlanningBeforeCodeEditing(PlanningBeforeCodeEditResponse),
    CodeEditTool(String),
    LSPDiagnostics(LSPDiagnosticsOutput),
    CodeToEdit(CodeToEditToolOutput),
    ReRankSnippets(ReRankEntriesForBroker),
    ImportantSymbols(CodeSymbolImportantResponse),
    GoToDefinition(GoToDefinitionResponse),
    GoToReference(GoToReferencesResponse),
    FileOpen(OpenFileResponse),
    GrepSingleFile(FindInFileResponse),
    GoToImplementation(GoToImplementationResponse),
    CodeToEditSnippets(CodeToEditFilterResponse),
    CodeToEditSingleSymbolSnippets(CodeToEditSymbolResponse),
    EditorApplyChanges(EditorApplyResponse),
    UtilityCodeSearch(CodeSymbolImportantResponse),
    GetQuickFixList(GetQuickFixResponse),
    LSPQuickFixInvoation(LSPQuickFixInvocationResponse),
    CodeCorrectnessAction(CodeCorrectnessAction),
    CodeEditingForError(String),
    ClassSymbolFollowupResponse(ClassSymbolFollowupResponse),
    // Probe requests
    ProbeCreateQuestionForSymbol(String),
    ProbeEnoughOrDeeper(ProbeEnoughOrDeeperResponse),
    ProbeSubSymbolFiltering(CodeToProbeSubSymbolList),
    ProbePossible(CodeSymbolShouldAskQuestionsResponse),
    ProbeQuestion(CodeSymbolToAskQuestionsResponse),
    ProbeSubSymbol(CodeToProbeFilterResponse),
    ProbeFollowAlongSymbol(ProbeNextSymbol),
    ProbeSummarizationResult(String),
    ProbeTryHardAnswer(String),
    // Repo map result
    RepoMapSearch(CodeSymbolImportantResponse),
    // SWE Bench test output
    SWEBenchTestOutput(SWEBenchTestRepsonse),
    // Test correction output
    TestCorrectionOutput(String),
    // Code Symbol follow for initial request
    CodeSymbolFollowForInitialRequest(CodeSymbolFollowInitialResponse),
    // New sub symbol creation
    NewSubSymbolCreation(NewSubSymbolRequiredResponse),
    // LSP symbol search information
    LSPSymbolSearchInformation(LSPGrepSymbolInCodebaseResponse),
    // Find the file for the symbol
    FindFileForNewSymbol(FindFileForSymbolResponse),
    // Find symbols to edit in the user context
    FindSymbolsToEditInContext(FindSymbolsToEditInContextResponse),
    // the outline nodes which we should use as context for the code editing
    ReRankedCodeSnippetsForCodeEditing(ReRankingSnippetsForCodeEditingResponse),
}

impl ToolOutput {
    pub fn re_ranked_code_snippets_for_editing_context(
        response: ReRankingSnippetsForCodeEditingResponse,
    ) -> Self {
        ToolOutput::ReRankedCodeSnippetsForCodeEditing(response)
    }

    pub fn find_symbols_to_edit_in_context(output: FindSymbolsToEditInContextResponse) -> Self {
        ToolOutput::FindSymbolsToEditInContext(output)
    }

    pub fn find_file_for_new_symbol(output: FindFileForSymbolResponse) -> Self {
        ToolOutput::FindFileForNewSymbol(output)
    }

    pub fn lsp_symbol_search_information(output: LSPGrepSymbolInCodebaseResponse) -> Self {
        ToolOutput::LSPSymbolSearchInformation(output)
    }

    pub fn new_sub_symbol_creation(output: NewSubSymbolRequiredResponse) -> Self {
        ToolOutput::NewSubSymbolCreation(output)
    }

    pub fn planning_before_code_editing(output: PlanningBeforeCodeEditResponse) -> Self {
        ToolOutput::PlanningBeforeCodeEditing(output)
    }

    pub fn code_symbol_follow_for_initial_request(output: CodeSymbolFollowInitialResponse) -> Self {
        ToolOutput::CodeSymbolFollowForInitialRequest(output)
    }

    pub fn swe_bench_test_output(output: SWEBenchTestRepsonse) -> Self {
        ToolOutput::SWEBenchTestOutput(output)
    }

    pub fn probe_summarization_result(response: String) -> Self {
        ToolOutput::ProbeSummarizationResult(response)
    }

    pub fn probe_follow_along_symbol(response: ProbeNextSymbol) -> Self {
        ToolOutput::ProbeFollowAlongSymbol(response)
    }

    pub fn probe_sub_symbol(response: CodeToProbeFilterResponse) -> Self {
        ToolOutput::ProbeSubSymbol(response)
    }

    pub fn probe_possible(response: CodeSymbolShouldAskQuestionsResponse) -> Self {
        ToolOutput::ProbePossible(response)
    }

    pub fn go_to_reference(refernece: GoToReferencesResponse) -> Self {
        ToolOutput::GoToReference(refernece)
    }

    pub fn code_correctness_action(output: CodeCorrectnessAction) -> Self {
        ToolOutput::CodeCorrectnessAction(output)
    }

    pub fn quick_fix_invocation_result(output: LSPQuickFixInvocationResponse) -> Self {
        ToolOutput::LSPQuickFixInvoation(output)
    }

    pub fn quick_fix_list(output: GetQuickFixResponse) -> Self {
        ToolOutput::GetQuickFixList(output)
    }

    pub fn code_edit_output(output: String) -> Self {
        ToolOutput::CodeEditTool(output)
    }

    pub fn lsp_diagnostics(diagnostics: LSPDiagnosticsOutput) -> Self {
        ToolOutput::LSPDiagnostics(diagnostics)
    }

    pub fn code_snippets_to_edit(output: CodeToEditToolOutput) -> Self {
        ToolOutput::CodeToEdit(output)
    }

    pub fn rerank_entries(reranked_snippets: ReRankEntriesForBroker) -> Self {
        ToolOutput::ReRankSnippets(reranked_snippets)
    }

    pub fn important_symbols(important_symbols: CodeSymbolImportantResponse) -> Self {
        ToolOutput::ImportantSymbols(important_symbols)
    }

    pub fn utility_code_symbols(important_symbols: CodeSymbolImportantResponse) -> Self {
        ToolOutput::UtilityCodeSearch(important_symbols)
    }

    pub fn go_to_definition(go_to_definition: GoToDefinitionResponse) -> Self {
        ToolOutput::GoToDefinition(go_to_definition)
    }

    pub fn file_open(file_open: OpenFileResponse) -> Self {
        ToolOutput::FileOpen(file_open)
    }

    pub fn go_to_implementation(go_to_implementation: GoToImplementationResponse) -> Self {
        ToolOutput::GoToImplementation(go_to_implementation)
    }

    pub fn get_quick_fix_actions(self) -> Option<GetQuickFixResponse> {
        match self {
            ToolOutput::GetQuickFixList(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_lsp_diagnostics(self) -> Option<LSPDiagnosticsOutput> {
        match self {
            ToolOutput::LSPDiagnostics(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_editor_apply_response(self) -> Option<EditorApplyResponse> {
        match self {
            ToolOutput::EditorApplyChanges(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_code_edit_output(self) -> Option<String> {
        match self {
            ToolOutput::CodeEditTool(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_important_symbols(self) -> Option<CodeSymbolImportantResponse> {
        match self {
            ToolOutput::ImportantSymbols(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_file_open_response(self) -> Option<OpenFileResponse> {
        match self {
            ToolOutput::FileOpen(file_open) => Some(file_open),
            _ => None,
        }
    }

    pub fn grep_single_file(self) -> Option<FindInFileResponse> {
        match self {
            ToolOutput::GrepSingleFile(grep_single_file) => Some(grep_single_file),
            _ => None,
        }
    }

    pub fn get_go_to_definition(self) -> Option<GoToDefinitionResponse> {
        match self {
            ToolOutput::GoToDefinition(go_to_definition) => Some(go_to_definition),
            _ => None,
        }
    }

    pub fn get_go_to_implementation(self) -> Option<GoToImplementationResponse> {
        match self {
            ToolOutput::GoToImplementation(result) => Some(result),
            _ => None,
        }
    }

    pub fn code_to_edit_filter(self) -> Option<CodeToEditFilterResponse> {
        match self {
            ToolOutput::CodeToEditSnippets(code_to_edit_filter) => Some(code_to_edit_filter),
            _ => None,
        }
    }

    pub fn code_to_edit_in_symbol(self) -> Option<CodeToEditSymbolResponse> {
        match self {
            ToolOutput::CodeToEditSingleSymbolSnippets(response) => Some(response),
            _ => None,
        }
    }

    pub fn utility_code_search_response(self) -> Option<CodeSymbolImportantResponse> {
        match self {
            ToolOutput::UtilityCodeSearch(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_test_correction_output(self) -> Option<String> {
        match self {
            ToolOutput::TestCorrectionOutput(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_code_correctness_action(self) -> Option<CodeCorrectnessAction> {
        match self {
            ToolOutput::CodeCorrectnessAction(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_quick_fix_invocation_result(self) -> Option<LSPQuickFixInvocationResponse> {
        match self {
            ToolOutput::LSPQuickFixInvoation(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_references(self) -> Option<GoToReferencesResponse> {
        match self {
            ToolOutput::GoToReference(output) => Some(output),
            _ => None,
        }
    }

    pub fn code_editing_for_error_fix(self) -> Option<String> {
        match self {
            ToolOutput::CodeEditingForError(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_swe_bench_test_output(self) -> Option<SWEBenchTestRepsonse> {
        match self {
            ToolOutput::SWEBenchTestOutput(output) => Some(output),
            _ => None,
        }
    }

    pub fn class_symbols_to_followup(self) -> Option<ClassSymbolFollowupResponse> {
        match self {
            ToolOutput::ClassSymbolFollowupResponse(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_probe_summarize_result(self) -> Option<String> {
        match self {
            ToolOutput::ProbeSummarizationResult(output) => Some(output),
            _ => None,
        }
    }

    pub fn get_probe_sub_symbol(self) -> Option<CodeToProbeFilterResponse> {
        match self {
            ToolOutput::ProbeSubSymbol(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_should_probe_symbol(self) -> Option<CodeSymbolShouldAskQuestionsResponse> {
        match self {
            ToolOutput::ProbePossible(request) => Some(request),
            _ => None,
        }
    }

    pub fn get_probe_symbol_deeper(self) -> Option<CodeSymbolToAskQuestionsResponse> {
        match self {
            ToolOutput::ProbeQuestion(request) => Some(request),
            _ => None,
        }
    }

    pub fn get_should_probe_next_symbol(self) -> Option<ProbeNextSymbol> {
        match self {
            ToolOutput::ProbeFollowAlongSymbol(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_code_symbol_follow_for_initial_request(
        self,
    ) -> Option<CodeSymbolFollowInitialResponse> {
        match self {
            ToolOutput::CodeSymbolFollowForInitialRequest(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_code_to_probe_sub_symbol_list(self) -> Option<CodeToProbeSubSymbolList> {
        match self {
            ToolOutput::ProbeSubSymbolFiltering(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_probe_enough_or_deeper(self) -> Option<ProbeEnoughOrDeeperResponse> {
        match self {
            ToolOutput::ProbeEnoughOrDeeper(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_probe_create_question_for_symbol(self) -> Option<String> {
        match self {
            ToolOutput::ProbeCreateQuestionForSymbol(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_plan_before_code_editing(self) -> Option<PlanningBeforeCodeEditResponse> {
        match self {
            ToolOutput::PlanningBeforeCodeEditing(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_new_sub_symbol_required(self) -> Option<NewSubSymbolRequiredResponse> {
        match self {
            ToolOutput::NewSubSymbolCreation(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_probe_try_harder_answer(self) -> Option<String> {
        match self {
            ToolOutput::ProbeTryHardAnswer(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_find_file_for_symbol_response(self) -> Option<FindFileForSymbolResponse> {
        match self {
            ToolOutput::FindFileForNewSymbol(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_lsp_grep_symbols_in_codebase_response(
        self,
    ) -> Option<LSPGrepSymbolInCodebaseResponse> {
        match self {
            ToolOutput::LSPSymbolSearchInformation(response) => Some(response),
            _ => None,
        }
    }

    pub fn get_code_symbols_to_edit_in_context(self) -> Option<FindSymbolsToEditInContextResponse> {
        match self {
            ToolOutput::FindSymbolsToEditInContext(response) => Some(response),
            _ => None,
        }
    }
}
