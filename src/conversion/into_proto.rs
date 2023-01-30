
use models;
use proto::client_service;
use std::str::FromStr;
use std::convert::From;

impl From<models::ApplicationConnection> for class_name::ApplicationConnection {
    fn from(i: models::ApplicationConnection) -> Self {
        let mut o = class_name::ApplicationConnection::new();
        o.set_ID(i.ID.into());
        o.set_FromApplicationID(i.FromApplicationID.into());
        o.set_ToApplicationID(i.ToApplicationID.into());
        o.set_RequiresCert(i.RequiresCert.into());
        o.set_RequiresKey(i.RequiresKey.into());
        o.set_RequiresAzureADAuth(i.RequiresAzureADAuth.into());
        o.set_ConnectionType(i.ConnectionType.to_string());
        o.set_DateAdded(i.DateAdded.to_string());
        o
    }
}

impl From<models::ApplicationNode> for class_name::ApplicationNode {
    fn from(i: models::ApplicationNode) -> Self {
        let mut o = class_name::ApplicationNode::new();
        o.set_ID(i.ID.into());
        o.set_OctopusID(i.OctopusID.into());
        o.set_ApplicationName(i.ApplicationName.to_string());
        o.set_ApplicationType(i.ApplicationType.to_string());
        o.set_IsIncludedInImminentRelease(i.IsIncludedInImminentRelease.into());
        o.set_ServerGroupDescription(i.ServerGroupDescription.to_string());
        o.set_DateAdded(i.DateAdded.to_string());
        o
    }
}

impl From<models::ApplicationTeam> for class_name::ApplicationTeam {
    fn from(i: models::ApplicationTeam) -> Self {
        let mut o = class_name::ApplicationTeam::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_IsActive(i.IsActive.into());
        o.set_SourceControlTeamID(i.SourceControlTeamID.to_string());
        o
    }
}

impl From<models::ApplicationUser> for class_name::ApplicationUser {
    fn from(i: models::ApplicationUser) -> Self {
        let mut o = class_name::ApplicationUser::new();
        o.set_ID(i.ID.into());
        o.set_First(i.First.to_string());
        o.set_Last(i.Last.to_string());
        o.set_ADUsername(i.ADUsername.to_string());
        o.set_Email(i.Email.to_string());
        o.set_PrimaryPhone(i.PrimaryPhone.to_string());
        o.set_SecondaryPhone(i.SecondaryPhone.to_string());
        o.set_ApplicationUserRoleID(i.ApplicationUserRoleID.into());
        o.set_IsActive(i.IsActive.into());
        o.set_ApplicationTeamID(i.ApplicationTeamID.into());
        o
    }
}

impl From<models::ApplicationUserReleaseApproval> for class_name::ApplicationUserReleaseApproval {
    fn from(i: models::ApplicationUserReleaseApproval) -> Self {
        let mut o = class_name::ApplicationUserReleaseApproval::new();
        o.set_ID(i.ID.into());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o.set_ReleaseApprovalTypeID(i.ReleaseApprovalTypeID.into());
        o
    }
}

impl From<models::ApplicationUserReleaseAvailability> for class_name::ApplicationUserReleaseAvailability {
    fn from(i: models::ApplicationUserReleaseAvailability) -> Self {
        let mut o = class_name::ApplicationUserReleaseAvailability::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseID(i.ReleaseID.into());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o.set_SupportAvailability(i.SupportAvailability.to_string());
        o
    }
}

impl From<models::ApplicationUserRole> for class_name::ApplicationUserRole {
    fn from(i: models::ApplicationUserRole) -> Self {
        let mut o = class_name::ApplicationUserRole::new();
        o.set_ID(i.ID.into());
        o.set_Role(i.Role.to_string());
        o
    }
}

impl From<models::ApplicationUserTeam> for class_name::ApplicationUserTeam {
    fn from(i: models::ApplicationUserTeam) -> Self {
        let mut o = class_name::ApplicationUserTeam::new();
        o.set_ID(i.ID.into());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o.set_ApplicationTeamID(i.ApplicationTeamID.into());
        o.set_IsMember(i.IsMember.into());
        o.set_IsSubscriber(i.IsSubscriber.into());
        o.set_IsAdmin(i.IsAdmin.into());
        o.set_IsDefault(i.IsDefault.into());
        o
    }
}

impl From<models::AuditReleaseActivityTask> for class_name::AuditReleaseActivityTask {
    fn from(i: models::AuditReleaseActivityTask) -> Self {
        let mut o = class_name::AuditReleaseActivityTask::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseActivityTaskID(i.ReleaseActivityTaskID.into());
        o.set_Title(i.Title.to_string());
        o.set_StageCategoryID(i.StageCategoryID.into());
        o.set_DeploymentInstructions(i.DeploymentInstructions.to_string());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_TargetEnvironmentID(i.TargetEnvironmentID.into());
        o.set_IsHidden(i.IsHidden.into());
        o.set_StageUserID(i.StageUserID.into());
        o.set_StageStatusID(i.StageStatusID.into());
        o.set_ProdUserID(i.ProdUserID.into());
        o.set_ProdStatusID(i.ProdStatusID.into());
        o.set_StageSortOrder(i.StageSortOrder.into());
        o.set_ProdSortOrder(i.ProdSortOrder.into());
        o.set_ProdCategoryID(i.ProdCategoryID.into());
        o.set_CanonicalOrder(i.CanonicalOrder.into());
        o.set_UpdateBy(i.UpdateBy.to_string());
        o.set_UpdatedOnDate(i.UpdatedOnDate.to_string());
        o.set_Action(i.Action.to_string());
        o.set_DependentTaskID(i.DependentTaskID.into());
        o
    }
}

impl From<models::BuildHistory> for class_name::BuildHistory {
    fn from(i: models::BuildHistory) -> Self {
        let mut o = class_name::BuildHistory::new();
        o.set_ID(i.ID.into());
        o.set_BuildProjectID(i.BuildProjectID.into());
        o.set_MetricsDailyCacheID(i.MetricsDailyCacheID.into());
        o.set_Total(i.Total.into());
        o.set_TotalSuccess(i.TotalSuccess.into());
        o.set_TotalFailed(i.TotalFailed.into());
        o
    }
}

impl From<models::BuildProject> for class_name::BuildProject {
    fn from(i: models::BuildProject) -> Self {
        let mut o = class_name::BuildProject::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_BuildDefinitionID(i.BuildDefinitionID.into());
        o.set_Path(i.Path.to_string());
        o
    }
}

impl From<models::Certificate> for class_name::Certificate {
    fn from(i: models::Certificate) -> Self {
        let mut o = class_name::Certificate::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_CreatedDate(i.CreatedDate.to_string());
        o.set_ExpirationDate(i.ExpirationDate.to_string());
        o.set_Thumbprint(i.Thumbprint.to_string());
        o.set_EnvironmentID(i.EnvironmentID.into());
        o.set_LastModifiedDateTime(i.LastModifiedDateTime.to_string());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_SubjectCName(i.SubjectCName.to_string());
        o.set_MarkForDeletion(i.MarkForDeletion.into());
        o.set_ReplacementSteps(i.ReplacementSteps.to_string());
        o.set_SerialNumber(i.SerialNumber.to_string());
        o.set_Source(i.Source.to_string());
        o.set_WorkItemID(i.WorkItemID.into());
        o.set_PreviousWorkItemIDs(i.PreviousWorkItemIDs.to_string());
        o.set_RequiresDowntime(i.RequiresDowntime.to_string());
        o
    }
}

impl From<models::DeploymentHistory> for class_name::DeploymentHistory {
    fn from(i: models::DeploymentHistory) -> Self {
        let mut o = class_name::DeploymentHistory::new();
        o.set_ID(i.ID.into());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_MetricsDailyCacheID(i.MetricsDailyCacheID.into());
        o.set_Total(i.Total.into());
        o.set_TotalSuccess(i.TotalSuccess.into());
        o.set_TotalFailed(i.TotalFailed.into());
        o
    }
}

impl From<models::ERTHistory> for class_name::ERTHistory {
    fn from(i: models::ERTHistory) -> Self {
        let mut o = class_name::ERTHistory::new();
        o.set_ID(i.ID.into());
        o.set_LogDate(i.LogDate.into());
        o.set_ApplicationTeamID(i.ApplicationTeamID.into());
        o.set_StartTime(i.StartTime.to_string());
        o.set_EndTime(i.EndTime.to_string());
        o.set_Status(i.Status.into());
        o
    }
}

impl From<models::ERTProject> for class_name::ERTProject {
    fn from(i: models::ERTProject) -> Self {
        let mut o = class_name::ERTProject::new();
        o.set_ID(i.ID.into());
        o.set_ERTHistoryID(i.ERTHistoryID.into());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_OctopusDeployProjectID(i.OctopusDeployProjectID.to_string());
        o.set_DeploymentID(i.DeploymentID.to_string());
        o.set_Version(i.Version.to_string());
        o.set_Status(i.Status.into());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o.set_Order(i.Order.into());
        o
    }
}

impl From<models::Environment> for class_name::Environment {
    fn from(i: models::Environment) -> Self {
        let mut o = class_name::Environment::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_Subscription(i.Subscription.to_string());
        o
    }
}

impl From<models::Export> for class_name::Export {
    fn from(i: models::Export) -> Self {
        let mut o = class_name::Export::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_StoredProcedureName(i.StoredProcedureName.to_string());
        o.set_Category(i.Category.to_string());
        o.set_IsActive(i.IsActive.into());
        o
    }
}

impl From<models::FunctionalTest> for class_name::FunctionalTest {
    fn from(i: models::FunctionalTest) -> Self {
        let mut o = class_name::FunctionalTest::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o
    }
}

impl From<models::FunctionalTestResult> for class_name::FunctionalTestResult {
    fn from(i: models::FunctionalTestResult) -> Self {
        let mut o = class_name::FunctionalTestResult::new();
        o.set_ID(i.ID.into());
        o.set_FunctionalTestID(i.FunctionalTestID.into());
        o.set_TotalTestsExecuted(i.TotalTestsExecuted.into());
        o.set_TotalTestsPassed(i.TotalTestsPassed.into());
        o.set_TotalTestsFailed(i.TotalTestsFailed.into());
        o.set_ResultFileLocation(i.ResultFileLocation.to_string());
        o.set_EnvironmentID(i.EnvironmentID.into());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_OctopusDeploymentID(i.OctopusDeploymentID.to_string());
        o.set_ExecutionResult(i.ExecutionResult.to_string());
        o.set_ExecutionDate(i.ExecutionDate.to_string());
        o
    }
}

impl From<models::MetricsBug> for class_name::MetricsBug {
    fn from(i: models::MetricsBug) -> Self {
        let mut o = class_name::MetricsBug::new();
        o.set_ID(i.ID.into());
        o.set_AzureDevOpsID(i.AzureDevOpsID.into());
        o.set_Title(i.Title.to_string());
        o.set_State(i.State.to_string());
        o.set_AreaPath(i.AreaPath.to_string());
        o.set_IterationID(i.IterationID.into());
        o.set_IterationPath(i.IterationPath.to_string());
        o.set_Priority(i.Priority.to_string());
        o.set_Severity(i.Severity.to_string());
        o.set_CreatedDate(i.CreatedDate.to_string());
        o.set_CreatedByDisplayName(i.CreatedByDisplayName.to_string());
        o.set_DiscoveredInEnvironment(i.DiscoveredInEnvironment.to_string());
        o.set_ActivatedDate(i.ActivatedDate.to_string());
        o.set_ActivatedByDisplayName(i.ActivatedByDisplayName.to_string());
        o.set_AssignedToDisplayName(i.AssignedToDisplayName.to_string());
        o.set_ChangedDate(i.ChangedDate.to_string());
        o.set_ChangedByDisplayName(i.ChangedByDisplayName.to_string());
        o.set_ResolvedDate(i.ResolvedDate.to_string());
        o.set_ResolvedByDisplayName(i.ResolvedByDisplayName.to_string());
        o.set_ClosedDate(i.ClosedDate.to_string());
        o.set_ClosedByDisplayName(i.ClosedByDisplayName.to_string());
        o.set_CommentCount(i.CommentCount.into());
        o.set_Tags(i.Tags.to_string());
        o.set_RootCause(i.RootCause.to_string());
        o.set_FunctionalArea(i.FunctionalArea.to_string());
        o.set_FinanciallyRelevant(i.FinanciallyRelevant.into());
        o.set_AffectsEnvironmentStability(i.AffectsEnvironmentStability.into());
        o.set_Vendor(i.Vendor.to_string());
        o.set_ProdSupportTicketID(i.ProdSupportTicketID.to_string());
        o
    }
}

impl From<models::MetricsDailyCache> for class_name::MetricsDailyCache {
    fn from(i: models::MetricsDailyCache) -> Self {
        let mut o = class_name::MetricsDailyCache::new();
        o.set_ID(i.ID.into());
        o.set_BuildTotal(i.BuildTotal.into());
        o.set_BuildSuccessTotal(i.BuildSuccessTotal.into());
        o.set_BuildFailureTotal(i.BuildFailureTotal.into());
        o.set_BuildSuccessDecimal(i.BuildSuccessDecimal.into());
        o.set_UnitTestTotal(i.UnitTestTotal.into());
        o.set_UnitTestSuccessTotal(i.UnitTestSuccessTotal.into());
        o.set_UnitTestFailureTotal(i.UnitTestFailureTotal.into());
        o.set_UnitTestSuccessDecimal(i.UnitTestSuccessDecimal.into());
        o.set_CodeCoverageAverageDecimal(i.CodeCoverageAverageDecimal.into());
        o.set_DeploymentTotal(i.DeploymentTotal.into());
        o.set_DeploymentSuccessTotal(i.DeploymentSuccessTotal.into());
        o.set_DeploymentFailureTotal(i.DeploymentFailureTotal.into());
        o.set_DeploymentSuccessDecimal(i.DeploymentSuccessDecimal.into());
        o.set_FunctionalTestTotal(i.FunctionalTestTotal.into());
        o.set_FunctionalTestSuccessTotal(i.FunctionalTestSuccessTotal.into());
        o.set_FunctionalTestFailureTotal(i.FunctionalTestFailureTotal.into());
        o.set_FunctionalTestSuccessDecimal(i.FunctionalTestSuccessDecimal.into());
        o.set_IntegrationTestProjectTotal(i.IntegrationTestProjectTotal.into());
        o.set_IntegrationTestSuccessTotal(i.IntegrationTestSuccessTotal.into());
        o.set_IntegrationTestFailureTotal(i.IntegrationTestFailureTotal.into());
        o.set_IntegrationTestSuccessDecimal(i.IntegrationTestSuccessDecimal.into());
        o.set_LogDate(i.LogDate.to_string());
        o.set_CodeCoverageProjectsTotal(i.CodeCoverageProjectsTotal.into());
        o
    }
}

impl From<models::MetricsPullRequest> for class_name::MetricsPullRequest {
    fn from(i: models::MetricsPullRequest) -> Self {
        let mut o = class_name::MetricsPullRequest::new();
        o.set_ID(i.ID.into());
        o.set_AzureDevOpsID(i.AzureDevOpsID.into());
        o.set_Title(i.Title.to_string());
        o.set_CreationDate(i.CreationDate.to_string());
        o.set_CreatedByDisplayName(i.CreatedByDisplayName.to_string());
        o.set_CreatedByUniqueIDentifier(i.CreatedByUniqueIDentifier.to_string());
        o.set_FirstEngagementDate(i.FirstEngagementDate.to_string());
        o.set_ApprovalDate(i.ApprovalDate.to_string());
        o.set_ApprovedByDisplayName(i.ApprovedByDisplayName.to_string());
        o.set_ApprovedByUniqueIDentifier(i.ApprovedByUniqueIDentifier.to_string());
        o.set_ClosedDate(i.ClosedDate.to_string());
        o.set_ClosedByDisplayName(i.ClosedByDisplayName.to_string());
        o.set_ClosedByUniqueIDentifier(i.ClosedByUniqueIDentifier.to_string());
        o.set_RepositoryName(i.RepositoryName.to_string());
        o.set_RepositoryID(i.RepositoryID.to_string());
        o.set_Status(i.Status.to_string());
        o.set_SourceBranchName(i.SourceBranchName.to_string());
        o.set_TargetBranchName(i.TargetBranchName.to_string());
        o.set_MergeStatus(i.MergeStatus.to_string());
        o.set_MergeID(i.MergeID.to_string());
        o.set_NumberOfOriginalCommits(i.NumberOfOriginalCommits.into());
        o.set_NumberOfAdditionalCommits(i.NumberOfAdditionalCommits.into());
        o.set_TotalCommits(i.TotalCommits.into());
        o.set_TotalCommentThreadCount(i.TotalCommentThreadCount.into());
        o.set_CommentThreadsBeforeApproval(i.CommentThreadsBeforeApproval.into());
        o.set_CommentThreadsAfterApproval(i.CommentThreadsAfterApproval.into());
        o.set_TotalCommentCount(i.TotalCommentCount.into());
        o.set_MostCommentsInSingleThread(i.MostCommentsInSingleThread.into());
        o
    }
}

impl From<models::NotificationQueue> for class_name::NotificationQueue {
    fn from(i: models::NotificationQueue) -> Self {
        let mut o = class_name::NotificationQueue::new();
        o.set_ID(i.ID.into());
        o.set_Type(i.Type.to_string());
        o.set_From_(i.From_.to_string());
        o.set_To(i.To.to_string());
        o.set_CC(i.CC.to_string());
        o.set_BCC(i.BCC.to_string());
        o.set_Subject(i.Subject.to_string());
        o.set_Body(i.Body.to_string());
        o.set_HasBeenSent(i.HasBeenSent.into());
        o
    }
}

impl From<models::OctopusProject> for class_name::OctopusProject {
    fn from(i: models::OctopusProject) -> Self {
        let mut o = class_name::OctopusProject::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_OctopusProjectID(i.OctopusProjectID.to_string());
        o.set_OctopusProjectGroupID(i.OctopusProjectGroupID.to_string());
        o.set_IsEnabled(i.IsEnabled.into());
        o
    }
}

impl From<models::ProjectCodeCoverage> for class_name::ProjectCodeCoverage {
    fn from(i: models::ProjectCodeCoverage) -> Self {
        let mut o = class_name::ProjectCodeCoverage::new();
        o.set_ID(i.ID.into());
        o.set_BuildProjectID(i.BuildProjectID.into());
        o.set_CodeCoverageValue(i.CodeCoverageValue.into());
        o.set_CodeCoverageDate(i.CodeCoverageDate.to_string());
        o.set_ReleaseNotesPath(i.ReleaseNotesPath.to_string());
        o
    }
}

impl From<models::ProjectRollUp> for class_name::ProjectRollUp {
    fn from(i: models::ProjectRollUp) -> Self {
        let mut o = class_name::ProjectRollUp::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o
    }
}

impl From<models::ProjectRollUpOctopusProject> for class_name::ProjectRollUpOctopusProject {
    fn from(i: models::ProjectRollUpOctopusProject) -> Self {
        let mut o = class_name::ProjectRollUpOctopusProject::new();
        o.set_ID(i.ID.into());
        o.set_ProjectRollUpID(i.ProjectRollUpID.into());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o
    }
}

impl From<models::Release> for class_name::Release {
    fn from(i: models::Release) -> Self {
        let mut o = class_name::Release::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_ReleaseDate(i.ReleaseDate.to_string());
        o.set_IsOffCycle(i.IsOffCycle.into());
        o.set_ReleaseStatusID(i.ReleaseStatusID.into());
        o.set_DowntimeNotes(i.DowntimeNotes.to_string());
        o.set_ReleaseCommitDate(i.ReleaseCommitDate.to_string());
        o.set_RegressionQueryLink(i.RegressionQueryLink.to_string());
        o.set_Description(i.Description.to_string());
        o.set_ChangeControlNumber(i.ChangeControlNumber.to_string());
        o.set_TotalWorkItemsTaggedForRelease(i.TotalWorkItemsTaggedForRelease.into());
        o.set_IsReadyForQa(i.IsReadyForQa.into());
        o
    }
}

impl From<models::ReleaseActivity> for class_name::ReleaseActivity {
    fn from(i: models::ReleaseActivity) -> Self {
        let mut o = class_name::ReleaseActivity::new();
        o.set_ID(i.ID.into());
        o.set_Title(i.Title.to_string());
        o.set_ReleaseID(i.ReleaseID.into());
        o.set_CreatedBy(i.CreatedBy.to_string());
        o.set_CreatedDate(i.CreatedDate.to_string());
        o.set_IsCompliant(i.IsCompliant.into());
        o.set_BackOutProcedures(i.BackOutProcedures.to_string());
        o.set_JustificationAndPriority(i.JustificationAndPriority.to_string());
        o.set_ProductionValidation(i.ProductionValidation.to_string());
        o.set_Risk(i.Risk.to_string());
        o.set_RiskLevel(i.RiskLevel.to_string());
        o.set_PriorityLevel(i.PriorityLevel.to_string());
        o.set_RequiresDowntime(i.RequiresDowntime.to_string());
        o.set_RequiresPerformanceTesting(i.RequiresPerformanceTesting.to_string());
        o.set_ApplicationTeamID(i.ApplicationTeamID.into());
        o.set_LastModifiedBy(i.LastModifiedBy.to_string());
        o.set_LastModifiedDate(i.LastModifiedDate.to_string());
        o.set_ChangeManagementID(i.ChangeManagementID.to_string());
        o.set_ExceptionReason(i.ExceptionReason.to_string());
        o.set_ExceptionGranted(i.ExceptionGranted.into());
        o.set_JiraWorkItems(i.JiraWorkItems.to_string());
        o
    }
}

impl From<models::ReleaseActivityApproval> for class_name::ReleaseActivityApproval {
    fn from(i: models::ReleaseActivityApproval) -> Self {
        let mut o = class_name::ReleaseActivityApproval::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseActivityID(i.ReleaseActivityID.into());
        o.set_ReleaseApprovalTypeID(i.ReleaseApprovalTypeID.into());
        o.set_RiskAssessment(i.RiskAssessment.to_string());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o.set_CreatedDate(i.CreatedDate.to_string());
        o.set_Status(i.Status.to_string());
        o.set_Comments(i.Comments.to_string());
        o
    }
}

impl From<models::ReleaseActivityFeature> for class_name::ReleaseActivityFeature {
    fn from(i: models::ReleaseActivityFeature) -> Self {
        let mut o = class_name::ReleaseActivityFeature::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseActivityID(i.ReleaseActivityID.into());
        o.set_FeatureID(i.FeatureID.into());
        o.set_TeamID(i.TeamID.to_string());
        o.set_ParentID(i.ParentID.into());
        o.set_ParentTitle(i.ParentTitle.to_string());
        o.set_FeatureTitle(i.FeatureTitle.to_string());
        o.set_TargetDate(i.TargetDate.into());
        o
    }
}

impl From<models::ReleaseActivityRelatedApplicationUser> for class_name::ReleaseActivityRelatedApplicationUser {
    fn from(i: models::ReleaseActivityRelatedApplicationUser) -> Self {
        let mut o = class_name::ReleaseActivityRelatedApplicationUser::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseActivityID(i.ReleaseActivityID.into());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o
    }
}

impl From<models::ReleaseActivityRelatedTask> for class_name::ReleaseActivityRelatedTask {
    fn from(i: models::ReleaseActivityRelatedTask) -> Self {
        let mut o = class_name::ReleaseActivityRelatedTask::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseActivityID(i.ReleaseActivityID.into());
        o.set_ReleaseActivityTaskID(i.ReleaseActivityTaskID.into());
        o.set_OctopusProjectSelectedVersion(i.OctopusProjectSelectedVersion.to_string());
        o
    }
}

impl From<models::ReleaseActivityTask> for class_name::ReleaseActivityTask {
    fn from(i: models::ReleaseActivityTask) -> Self {
        let mut o = class_name::ReleaseActivityTask::new();
        o.set_ID(i.ID.into());
        o.set_Title(i.Title.to_string());
        o.set_StageCategoryID(i.StageCategoryID.into());
        o.set_DeploymentInstructions(i.DeploymentInstructions.to_string());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_TargetEnvironmentID(i.TargetEnvironmentID.into());
        o.set_IsHidden(i.IsHidden.into());
        o.set_StageStatusID(i.StageStatusID.into());
        o.set_ProdUserID(i.ProdUserID.into());
        o.set_StageUserID(i.StageUserID.into());
        o.set_ProdStatusID(i.ProdStatusID.into());
        o.set_StageSortOrder(i.StageSortOrder.into());
        o.set_ProdSortOrder(i.ProdSortOrder.into());
        o.set_ProdCategoryID(i.ProdCategoryID.into());
        o.set_CanonicalOrder(i.CanonicalOrder.into());
        o.set_LastModifiedBy(i.LastModifiedBy.to_string());
        o.set_LastModifiedDateTime(i.LastModifiedDateTime.to_string());
        o.set_DependentTaskID(i.DependentTaskID.into());
        o.set_OctopusProjectSelectedVersion(i.OctopusProjectSelectedVersion.to_string());
        o
    }
}

impl From<models::ReleaseActivityTaskAttachment> for class_name::ReleaseActivityTaskAttachment {
    fn from(i: models::ReleaseActivityTaskAttachment) -> Self {
        let mut o = class_name::ReleaseActivityTaskAttachment::new();
        o.set_ID(i.ID.to_string());
        o.set_ReleaseActivityTaskID(i.ReleaseActivityTaskID.into());
        o.set_File(i.File.into());
        o.set_FileName(i.FileName.to_string());
        o.set_ContentType(i.ContentType.to_string());
        o
    }
}

impl From<models::ReleaseActivityTaskCategory> for class_name::ReleaseActivityTaskCategory {
    fn from(i: models::ReleaseActivityTaskCategory) -> Self {
        let mut o = class_name::ReleaseActivityTaskCategory::new();
        o.set_ID(i.ID.into());
        o.set_Category(i.Category.to_string());
        o.set_SortOrder(i.SortOrder.into());
        o.set_IsActive(i.IsActive.into());
        o
    }
}

impl From<models::ReleaseActivityTaskMessageQueue> for class_name::ReleaseActivityTaskMessageQueue {
    fn from(i: models::ReleaseActivityTaskMessageQueue) -> Self {
        let mut o = class_name::ReleaseActivityTaskMessageQueue::new();
        o.set_ID(i.ID.into());
        o.set_IsProcessed(i.IsProcessed.into());
        o.set_HasBeenSeen(i.HasBeenSeen.into());
        o.set_HasBeenPeeked(i.HasBeenPeeked.into());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o.set_ReleaseActivityTaskID(i.ReleaseActivityTaskID.into());
        o.set_LastModifiedBy(i.LastModifiedBy.to_string());
        o.set_LastModifiedDateTime(i.LastModifiedDateTime.to_string());
        o.set_ReleaseEnvironment(i.ReleaseEnvironment.to_string());
        o
    }
}

impl From<models::ReleaseActivityTaskStatu> for class_name::ReleaseActivityTaskStatu {
    fn from(i: models::ReleaseActivityTaskStatu) -> Self {
        let mut o = class_name::ReleaseActivityTaskStatu::new();
        o.set_ID(i.ID.into());
        o.set_Status(i.Status.to_string());
        o
    }
}

impl From<models::ReleaseActivityTaskTargetEnvironment> for class_name::ReleaseActivityTaskTargetEnvironment {
    fn from(i: models::ReleaseActivityTaskTargetEnvironment) -> Self {
        let mut o = class_name::ReleaseActivityTaskTargetEnvironment::new();
        o.set_ID(i.ID.into());
        o.set_TargetEnvironment(i.TargetEnvironment.to_string());
        o
    }
}

impl From<models::ReleaseApproval> for class_name::ReleaseApproval {
    fn from(i: models::ReleaseApproval) -> Self {
        let mut o = class_name::ReleaseApproval::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseID(i.ReleaseID.into());
        o.set_ReleaseApprovalTypeID(i.ReleaseApprovalTypeID.into());
        o.set_RiskAssessment(i.RiskAssessment.to_string());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o.set_CreatedDate(i.CreatedDate.to_string());
        o.set_Status(i.Status.to_string());
        o.set_Comments(i.Comments.to_string());
        o
    }
}

impl From<models::ReleaseApprovalStatistic> for class_name::ReleaseApprovalStatistic {
    fn from(i: models::ReleaseApprovalStatistic) -> Self {
        let mut o = class_name::ReleaseApprovalStatistic::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseApprovalID(i.ReleaseApprovalID.into());
        o.set_Comments(i.Comments.to_string());
        o.set_Value(i.Value.to_string());
        o
    }
}

impl From<models::ReleaseApprovalType> for class_name::ReleaseApprovalType {
    fn from(i: models::ReleaseApprovalType) -> Self {
        let mut o = class_name::ReleaseApprovalType::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_IsActive(i.IsActive.into());
        o.set_DisplayOrder(i.DisplayOrder.into());
        o.set_Icon(i.Icon.to_string());
        o.set_Description(i.Description.to_string());
        o
    }
}

impl From<models::ReleaseBranchHistory> for class_name::ReleaseBranchHistory {
    fn from(i: models::ReleaseBranchHistory) -> Self {
        let mut o = class_name::ReleaseBranchHistory::new();
        o.set_ID(i.ID.into());
        o.set_OctopusProjectID(i.OctopusProjectID.into());
        o.set_ReleaseID(i.ReleaseID.into());
        o.set_CreatedDate(i.CreatedDate.to_string());
        o.set_CreatedBy(i.CreatedBy.to_string());
        o.set_VersionNumber(i.VersionNumber.to_string());
        o
    }
}

impl From<models::ReleaseRelatedApplicationUser> for class_name::ReleaseRelatedApplicationUser {
    fn from(i: models::ReleaseRelatedApplicationUser) -> Self {
        let mut o = class_name::ReleaseRelatedApplicationUser::new();
        o.set_ID(i.ID.into());
        o.set_ReleaseID(i.ReleaseID.into());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o
    }
}

impl From<models::ReleaseRelatedCategory> for class_name::ReleaseRelatedCategory {
    fn from(i: models::ReleaseRelatedCategory) -> Self {
        let mut o = class_name::ReleaseRelatedCategory::new();
        o.set_ID(i.ID.into());
        o.set_Category(i.Category.to_string());
        o.set_ReleaseID(i.ReleaseID.into());
        o.set_SortOrder(i.SortOrder.into());
        o
    }
}

impl From<models::ReleaseStatu> for class_name::ReleaseStatu {
    fn from(i: models::ReleaseStatu) -> Self {
        let mut o = class_name::ReleaseStatu::new();
        o.set_ID(i.ID.into());
        o.set_Status(i.Status.to_string());
        o
    }
}

impl From<models::RequestPathway> for class_name::RequestPathway {
    fn from(i: models::RequestPathway) -> Self {
        let mut o = class_name::RequestPathway::new();
        o.set_ID(i.ID.into());
        o.set_ToApplicationID(i.ToApplicationID.into());
        o.set_FromApplicationID(i.FromApplicationID.into());
        o.set_ConnectionType(i.ConnectionType.to_string());
        o
    }
}

impl From<models::ResourceCost> for class_name::ResourceCost {
    fn from(i: models::ResourceCost) -> Self {
        let mut o = class_name::ResourceCost::new();
        o.set_ID(i.ID.into());
        o.set_Subscription(i.Subscription.to_string());
        o.set_ResourceGroupName(i.ResourceGroupName.to_string());
        o.set_ResourceGroupResourceName(i.ResourceGroupResourceName.to_string());
        o.set_CostDate(i.CostDate.into());
        o.set_Cost(i.Cost.to_string());
        o
    }
}

impl From<models::ResourceGroupCost> for class_name::ResourceGroupCost {
    fn from(i: models::ResourceGroupCost) -> Self {
        let mut o = class_name::ResourceGroupCost::new();
        o.set_ID(i.ID.into());
        o.set_Subscription(i.Subscription.to_string());
        o.set_ResourceGroupName(i.ResourceGroupName.to_string());
        o.set_CostDate(i.CostDate.into());
        o.set_Cost(i.Cost.to_string());
        o
    }
}

impl From<models::STAR> for class_name::STAR {
    fn from(i: models::STAR) -> Self {
        let mut o = class_name::STAR::new();
        o.set_ID(i.ID.into());
        o.set_DateCreated(i.DateCreated.to_string());
        o
    }
}

impl From<models::STARData> for class_name::STARData {
    fn from(i: models::STARData) -> Self {
        let mut o = class_name::STARData::new();
        o.set_ID(i.ID.into());
        o.set_StarID(i.StarID.into());
        o.set_Record(i.Record.to_string());
        o
    }
}

impl From<models::SlottingMigrationLog> for class_name::SlottingMigrationLog {
    fn from(i: models::SlottingMigrationLog) -> Self {
        let mut o = class_name::SlottingMigrationLog::new();
        o.set_ID(i.ID.into());
        o.set_OctopusProjectID(i.OctopusProjectID.to_string());
        o.set_ApplicationUserID(i.ApplicationUserID.into());
        o
    }
}

impl From<models::SystemEventLog> for class_name::SystemEventLog {
    fn from(i: models::SystemEventLog) -> Self {
        let mut o = class_name::SystemEventLog::new();
        o.set_ID(i.ID.into());
        o.set_Source(i.Source.to_string());
        o.set_Description(i.Description.to_string());
        o.set_OctopusProjectID(i.OctopusProjectID.to_string());
        o.set_EventDate(i.EventDate.to_string());
        o
    }
}

impl From<models::SystemValue> for class_name::SystemValue {
    fn from(i: models::SystemValue) -> Self {
        let mut o = class_name::SystemValue::new();
        o.set_ID(i.ID.into());
        o.set_Name(i.Name.to_string());
        o.set_Value(i.Value.to_string());
        o.set_Date(i.Date.to_string());
        o
    }
}

impl From<models::UnitTestHistory> for class_name::UnitTestHistory {
    fn from(i: models::UnitTestHistory) -> Self {
        let mut o = class_name::UnitTestHistory::new();
        o.set_ID(i.ID.into());
        o.set_BuildProjectID(i.BuildProjectID.into());
        o.set_MetricsDailyCacheID(i.MetricsDailyCacheID.into());
        o.set_Total(i.Total.into());
        o.set_TotalSuccess(i.TotalSuccess.into());
        o.set_TotalFailed(i.TotalFailed.into());
        o
    }
}

impl From<models::VstsFeatureCompliance> for class_name::VstsFeatureCompliance {
    fn from(i: models::VstsFeatureCompliance) -> Self {
        let mut o = class_name::VstsFeatureCompliance::new();
        o.set_ID(i.ID.into());
        o.set_FeatureID(i.FeatureID.into());
        o.set_ReleaseName(i.ReleaseName.to_string());
        o.set_IsCompliant(i.IsCompliant.into());
        o.set_NumberNonCompliantChildren(i.NumberNonCompliantChildren.into());
        o.set_LastCheckedDate(i.LastCheckedDate.to_string());
        o
    }
}

