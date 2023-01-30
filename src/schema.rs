// @generated automatically by Diesel CLI.

diesel::table! {
    ApplicationConnection (ID) {
        ID -> Integer,
        FromApplicationID -> Integer,
        ToApplicationID -> Integer,
        RequiresCert -> Bool,
        RequiresKey -> Bool,
        RequiresAzureADAuth -> Bool,
        ConnectionType -> Nullable<Text>,
        DateAdded -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ApplicationNode (ID) {
        ID -> Integer,
        OctopusID -> Integer,
        ApplicationName -> Text,
        ApplicationType -> Nullable<Text>,
        IsIncludedInImminentRelease -> Nullable<Bool>,
        ServerGroupDescription -> Nullable<Text>,
        DateAdded -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ApplicationTeam (ID) {
        ID -> Integer,
        Name -> Text,
        IsActive -> Nullable<Bool>,
        SourceControlTeamID -> Nullable<Text>,
    }
}

diesel::table! {
    ApplicationUser (ID) {
        ID -> Integer,
        First -> Text,
        Last -> Text,
        ADUsername -> Text,
        Email -> Text,
        PrimaryPhone -> Nullable<Text>,
        SecondaryPhone -> Nullable<Text>,
        ApplicationUserRoleID -> Integer,
        IsActive -> Nullable<Integer>,
        ApplicationTeamID -> Nullable<Integer>,
    }
}

diesel::table! {
    ApplicationUserReleaseApproval (ID) {
        ID -> Integer,
        ApplicationUserID -> Integer,
        ReleaseApprovalTypeID -> Integer,
    }
}

diesel::table! {
    ApplicationUserReleaseAvailability (ID) {
        ID -> Integer,
        ReleaseID -> Integer,
        ApplicationUserID -> Integer,
        SupportAvailability -> Nullable<Text>,
    }
}

diesel::table! {
    ApplicationUserRole (ID) {
        ID -> Integer,
        Role -> Text,
    }
}

diesel::table! {
    ApplicationUserTeam (ID) {
        ID -> Integer,
        ApplicationUserID -> Integer,
        ApplicationTeamID -> Integer,
        IsMember -> Bool,
        IsSubscriber -> Bool,
        IsAdmin -> Bool,
        IsDefault -> Bool,
    }
}

diesel::table! {
    AuditReleaseActivityTask (ID) {
        ID -> Integer,
        ReleaseActivityTaskID -> Nullable<Integer>,
        Title -> Nullable<Text>,
        StageCategoryID -> Nullable<Integer>,
        DeploymentInstructions -> Nullable<Text>,
        OctopusProjectID -> Nullable<Integer>,
        TargetEnvironmentID -> Nullable<Integer>,
        IsHidden -> Nullable<Bool>,
        StageUserID -> Nullable<Integer>,
        StageStatusID -> Nullable<Integer>,
        ProdUserID -> Nullable<Integer>,
        ProdStatusID -> Nullable<Integer>,
        StageSortOrder -> Nullable<Integer>,
        ProdSortOrder -> Nullable<Integer>,
        ProdCategoryID -> Nullable<Integer>,
        CanonicalOrder -> Nullable<Integer>,
        UpdateBy -> Text,
        UpdatedOnDate -> Timestamp,
        Action -> Text,
        DependentTaskID -> Nullable<Integer>,
    }
}

diesel::table! {
    BuildHistory (ID) {
        ID -> Integer,
        BuildProjectID -> Integer,
        MetricsDailyCacheID -> Integer,
        Total -> Integer,
        TotalSuccess -> Integer,
        TotalFailed -> Integer,
    }
}

diesel::table! {
    BuildProject (ID) {
        ID -> Integer,
        Name -> Text,
        OctopusProjectID -> Nullable<Integer>,
        BuildDefinitionID -> Nullable<Integer>,
        Path -> Nullable<Text>,
    }
}

diesel::table! {
    Certificate (ID) {
        ID -> Integer,
        Name -> Text,
        CreatedDate -> Timestamp,
        ExpirationDate -> Timestamp,
        Thumbprint -> Nullable<Text>,
        EnvironmentID -> Nullable<Integer>,
        LastModifiedDateTime -> Timestamp,
        OctopusProjectID -> Nullable<Integer>,
        SubjectCName -> Nullable<Text>,
        MarkForDeletion -> Bool,
        ReplacementSteps -> Nullable<Text>,
        SerialNumber -> Nullable<Text>,
        Source -> Nullable<Text>,
        WorkItemID -> Nullable<Integer>,
        PreviousWorkItemIDs -> Nullable<Text>,
        RequiresDowntime -> Nullable<Text>,
    }
}

diesel::table! {
    DeploymentHistory (ID) {
        ID -> Integer,
        OctopusProjectID -> Integer,
        MetricsDailyCacheID -> Integer,
        Total -> Integer,
        TotalSuccess -> Integer,
        TotalFailed -> Integer,
    }
}

diesel::table! {
    ERTHistory (ID) {
        ID -> Integer,
        LogDate -> Date,
        ApplicationTeamID -> Nullable<Integer>,
        StartTime -> Nullable<Timestamp>,
        EndTime -> Nullable<Timestamp>,
        Status -> Integer,
    }
}

diesel::table! {
    ERTProject (ID) {
        ID -> Integer,
        ERTHistoryID -> Integer,
        OctopusProjectID -> Integer,
        OctopusDeployProjectID -> Text,
        DeploymentID -> Nullable<Text>,
        Version -> Nullable<Text>,
        Status -> Integer,
        ApplicationUserID -> Integer,
        Order -> Integer,
    }
}

diesel::table! {
    Environment (ID) {
        ID -> Integer,
        Name -> Text,
        Subscription -> Nullable<Text>,
    }
}

diesel::table! {
    Export (ID) {
        ID -> Integer,
        Name -> Text,
        StoredProcedureName -> Text,
        Category -> Text,
        IsActive -> Bool,
    }
}

diesel::table! {
    FunctionalTest (ID) {
        ID -> Integer,
        Name -> Text,
    }
}

diesel::table! {
    FunctionalTestResult (ID) {
        ID -> Integer,
        FunctionalTestID -> Integer,
        TotalTestsExecuted -> Integer,
        TotalTestsPassed -> Integer,
        TotalTestsFailed -> Integer,
        ResultFileLocation -> Text,
        EnvironmentID -> Integer,
        OctopusProjectID -> Integer,
        OctopusDeploymentID -> Nullable<Text>,
        ExecutionResult -> Text,
        ExecutionDate -> Timestamp,
    }
}

diesel::table! {
    MetricsBugs (ID) {
        ID -> Integer,
        AzureDevOpsID -> Integer,
        Title -> Text,
        State -> Text,
        AreaPath -> Nullable<Text>,
        IterationID -> Nullable<Integer>,
        IterationPath -> Nullable<Text>,
        Priority -> Nullable<Text>,
        Severity -> Nullable<Text>,
        CreatedDate -> Timestamp,
        CreatedByDisplayName -> Nullable<Text>,
        DiscoveredInEnvironment -> Nullable<Text>,
        ActivatedDate -> Nullable<Timestamp>,
        ActivatedByDisplayName -> Nullable<Text>,
        AssignedToDisplayName -> Nullable<Text>,
        ChangedDate -> Nullable<Timestamp>,
        ChangedByDisplayName -> Nullable<Text>,
        ResolvedDate -> Nullable<Timestamp>,
        ResolvedByDisplayName -> Nullable<Text>,
        ClosedDate -> Nullable<Timestamp>,
        ClosedByDisplayName -> Nullable<Text>,
        CommentCount -> Nullable<Integer>,
        Tags -> Nullable<Text>,
        RootCause -> Nullable<Text>,
        FunctionalArea -> Nullable<Text>,
        FinanciallyRelevant -> Nullable<Bool>,
        AffectsEnvironmentStability -> Nullable<Bool>,
        Vendor -> Nullable<Text>,
        ProdSupportTicketID -> Nullable<Text>,
    }
}

diesel::table! {
    MetricsDailyCache (ID) {
        ID -> Integer,
        BuildTotal -> Integer,
        BuildSuccessTotal -> Integer,
        BuildFailureTotal -> Integer,
        BuildSuccessDecimal -> Float,
        UnitTestTotal -> Integer,
        UnitTestSuccessTotal -> Integer,
        UnitTestFailureTotal -> Integer,
        UnitTestSuccessDecimal -> Float,
        CodeCoverageAverageDecimal -> Float,
        DeploymentTotal -> Integer,
        DeploymentSuccessTotal -> Integer,
        DeploymentFailureTotal -> Integer,
        DeploymentSuccessDecimal -> Float,
        FunctionalTestTotal -> Integer,
        FunctionalTestSuccessTotal -> Integer,
        FunctionalTestFailureTotal -> Integer,
        FunctionalTestSuccessDecimal -> Float,
        IntegrationTestProjectTotal -> Integer,
        IntegrationTestSuccessTotal -> Integer,
        IntegrationTestFailureTotal -> Integer,
        IntegrationTestSuccessDecimal -> Float,
        LogDate -> Timestamp,
        CodeCoverageProjectsTotal -> Nullable<Integer>,
    }
}

diesel::table! {
    MetricsPullRequests (ID) {
        ID -> Integer,
        AzureDevOpsID -> Integer,
        Title -> Text,
        CreationDate -> Timestamp,
        CreatedByDisplayName -> Text,
        CreatedByUniqueIDentifier -> Text,
        FirstEngagementDate -> Nullable<Timestamp>,
        ApprovalDate -> Nullable<Timestamp>,
        ApprovedByDisplayName -> Nullable<Text>,
        ApprovedByUniqueIDentifier -> Nullable<Text>,
        ClosedDate -> Nullable<Timestamp>,
        ClosedByDisplayName -> Nullable<Text>,
        ClosedByUniqueIDentifier -> Nullable<Text>,
        RepositoryName -> Text,
        RepositoryID -> Text,
        Status -> Text,
        SourceBranchName -> Text,
        TargetBranchName -> Text,
        MergeStatus -> Nullable<Text>,
        MergeID -> Nullable<Text>,
        NumberOfOriginalCommits -> Nullable<Integer>,
        NumberOfAdditionalCommits -> Nullable<Integer>,
        TotalCommits -> Nullable<Integer>,
        TotalCommentThreadCount -> Nullable<Integer>,
        CommentThreadsBeforeApproval -> Nullable<Integer>,
        CommentThreadsAfterApproval -> Nullable<Integer>,
        TotalCommentCount -> Nullable<Integer>,
        MostCommentsInSingleThread -> Nullable<Integer>,
    }
}

diesel::table! {
    NotificationQueue (ID) {
        ID -> Integer,
        Type -> Text,
        From_ -> Text,
        To -> Text,
        CC -> Text,
        BCC -> Text,
        Subject -> Text,
        Body -> Text,
        HasBeenSent -> Bool,
    }
}

diesel::table! {
    OctopusProject (ID) {
        ID -> Integer,
        Name -> Text,
        OctopusProjectID -> Text,
        OctopusProjectGroupID -> Text,
        IsEnabled -> Nullable<Bool>,
    }
}

diesel::table! {
    ProjectCodeCoverage (ID) {
        ID -> Integer,
        BuildProjectID -> Integer,
        CodeCoverageValue -> Float,
        CodeCoverageDate -> Timestamp,
        ReleaseNotesPath -> Nullable<Text>,
    }
}

diesel::table! {
    ProjectRollUp (ID) {
        ID -> Integer,
        Name -> Text,
    }
}

diesel::table! {
    ProjectRollUpOctopusProject (ID) {
        ID -> Integer,
        ProjectRollUpID -> Integer,
        OctopusProjectID -> Integer,
    }
}

diesel::table! {
    Release (ID) {
        ID -> Integer,
        Name -> Text,
        ReleaseDate -> Timestamp,
        IsOffCycle -> Nullable<Bool>,
        ReleaseStatusID -> Nullable<Integer>,
        DowntimeNotes -> Nullable<Text>,
        ReleaseCommitDate -> Text,
        RegressionQueryLink -> Nullable<Text>,
        Description -> Nullable<Text>,
        ChangeControlNumber -> Nullable<Text>,
        TotalWorkItemsTaggedForRelease -> Nullable<Integer>,
        IsReadyForQa -> Nullable<Bool>,
    }
}

diesel::table! {
    ReleaseActivity (ID) {
        ID -> Integer,
        Title -> Text,
        ReleaseID -> Nullable<Integer>,
        CreatedBy -> Nullable<Text>,
        CreatedDate -> Nullable<Timestamp>,
        IsCompliant -> Nullable<Bool>,
        BackOutProcedures -> Nullable<Text>,
        JustificationAndPriority -> Nullable<Text>,
        ProductionValidation -> Nullable<Text>,
        Risk -> Nullable<Text>,
        RiskLevel -> Nullable<Text>,
        PriorityLevel -> Nullable<Text>,
        RequiresDowntime -> Text,
        RequiresPerformanceTesting -> Text,
        ApplicationTeamID -> Nullable<Integer>,
        LastModifiedBy -> Nullable<Text>,
        LastModifiedDate -> Nullable<Timestamp>,
        ChangeManagementID -> Nullable<Text>,
        ExceptionReason -> Nullable<Text>,
        ExceptionGranted -> Nullable<Bool>,
        JiraWorkItems -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseActivityApproval (ID) {
        ID -> Integer,
        ReleaseActivityID -> Integer,
        ReleaseApprovalTypeID -> Nullable<Integer>,
        RiskAssessment -> Nullable<Text>,
        ApplicationUserID -> Nullable<Integer>,
        CreatedDate -> Nullable<Timestamp>,
        Status -> Nullable<Text>,
        Comments -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseActivityFeature (ID) {
        ID -> Integer,
        ReleaseActivityID -> Integer,
        FeatureID -> Integer,
        TeamID -> Nullable<Text>,
        ParentID -> Nullable<Integer>,
        ParentTitle -> Nullable<Text>,
        FeatureTitle -> Nullable<Text>,
        TargetDate -> Nullable<Date>,
    }
}

diesel::table! {
    ReleaseActivityRelatedApplicationUser (ID) {
        ID -> Integer,
        ReleaseActivityID -> Integer,
        ApplicationUserID -> Integer,
    }
}

diesel::table! {
    ReleaseActivityRelatedTask (ID) {
        ID -> Integer,
        ReleaseActivityID -> Integer,
        ReleaseActivityTaskID -> Integer,
        OctopusProjectSelectedVersion -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseActivityTask (ID) {
        ID -> Integer,
        Title -> Nullable<Text>,
        StageCategoryID -> Nullable<Integer>,
        DeploymentInstructions -> Nullable<Text>,
        OctopusProjectID -> Nullable<Integer>,
        TargetEnvironmentID -> Nullable<Integer>,
        IsHidden -> Nullable<Bool>,
        StageStatusID -> Nullable<Integer>,
        ProdUserID -> Nullable<Integer>,
        StageUserID -> Nullable<Integer>,
        ProdStatusID -> Nullable<Integer>,
        StageSortOrder -> Nullable<Integer>,
        ProdSortOrder -> Nullable<Integer>,
        ProdCategoryID -> Nullable<Integer>,
        CanonicalOrder -> Nullable<Integer>,
        LastModifiedBy -> Nullable<Text>,
        LastModifiedDateTime -> Nullable<Timestamp>,
        DependentTaskID -> Nullable<Integer>,
        OctopusProjectSelectedVersion -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseActivityTaskAttachment (ID) {
        ID -> Text,
        ReleaseActivityTaskID -> Integer,
        File -> Nullable<Binary>,
        FileName -> Nullable<Text>,
        ContentType -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseActivityTaskCategory (ID) {
        ID -> Integer,
        Category -> Nullable<Text>,
        SortOrder -> Nullable<Integer>,
        IsActive -> Nullable<Bool>,
    }
}

diesel::table! {
    ReleaseActivityTaskMessageQueue (ID) {
        ID -> Integer,
        IsProcessed -> Bool,
        HasBeenSeen -> Bool,
        HasBeenPeeked -> Bool,
        ApplicationUserID -> Integer,
        ReleaseActivityTaskID -> Integer,
        LastModifiedBy -> Nullable<Text>,
        LastModifiedDateTime -> Nullable<Timestamp>,
        ReleaseEnvironment -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseActivityTaskStatus (ID) {
        ID -> Integer,
        Status -> Text,
    }
}

diesel::table! {
    ReleaseActivityTaskTargetEnvironment (ID) {
        ID -> Integer,
        TargetEnvironment -> Text,
    }
}

diesel::table! {
    ReleaseApproval (ID) {
        ID -> Integer,
        ReleaseID -> Integer,
        ReleaseApprovalTypeID -> Integer,
        RiskAssessment -> Nullable<Text>,
        ApplicationUserID -> Integer,
        CreatedDate -> Nullable<Timestamp>,
        Status -> Nullable<Text>,
        Comments -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseApprovalStatistic (ID) {
        ID -> Integer,
        ReleaseApprovalID -> Integer,
        Comments -> Nullable<Text>,
        Value -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseApprovalType (ID) {
        ID -> Integer,
        Name -> Text,
        IsActive -> Nullable<Bool>,
        DisplayOrder -> Nullable<Integer>,
        Icon -> Nullable<Text>,
        Description -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseBranchHistory (ID) {
        ID -> Integer,
        OctopusProjectID -> Integer,
        ReleaseID -> Integer,
        CreatedDate -> Timestamp,
        CreatedBy -> Nullable<Text>,
        VersionNumber -> Nullable<Text>,
    }
}

diesel::table! {
    ReleaseRelatedApplicationUser (ID) {
        ID -> Integer,
        ReleaseID -> Integer,
        ApplicationUserID -> Integer,
    }
}

diesel::table! {
    ReleaseRelatedCategory (ID) {
        ID -> Integer,
        Category -> Nullable<Text>,
        ReleaseID -> Integer,
        SortOrder -> Integer,
    }
}

diesel::table! {
    ReleaseStatus (ID) {
        ID -> Integer,
        Status -> Text,
    }
}

diesel::table! {
    RequestPathways (ID) {
        ID -> Integer,
        ToApplicationID -> Nullable<Integer>,
        FromApplicationID -> Nullable<Integer>,
        ConnectionType -> Nullable<Text>,
    }
}

diesel::table! {
    ResourceCost (ID) {
        ID -> Integer,
        Subscription -> Text,
        ResourceGroupName -> Text,
        ResourceGroupResourceName -> Text,
        CostDate -> Nullable<Date>,
        Cost -> Text,
    }
}

diesel::table! {
    ResourceGroupCost (ID) {
        ID -> Integer,
        Subscription -> Text,
        ResourceGroupName -> Text,
        CostDate -> Nullable<Date>,
        Cost -> Text,
    }
}

diesel::table! {
    STAR (ID) {
        ID -> Integer,
        DateCreated -> Timestamp,
    }
}

diesel::table! {
    STARData (ID) {
        ID -> Integer,
        StarID -> Integer,
        Record -> Text,
    }
}

diesel::table! {
    SlottingMigrationLog (ID) {
        ID -> Integer,
        OctopusProjectID -> Nullable<Text>,
        ApplicationUserID -> Nullable<Integer>,
    }
}

diesel::table! {
    SystemEventLog (ID) {
        ID -> Integer,
        Source -> Text,
        Description -> Text,
        OctopusProjectID -> Nullable<Text>,
        EventDate -> Timestamp,
    }
}

diesel::table! {
    SystemValues (ID) {
        ID -> Integer,
        Name -> Text,
        Value -> Text,
        Date -> Timestamp,
    }
}

diesel::table! {
    UnitTestHistory (ID) {
        ID -> Integer,
        BuildProjectID -> Integer,
        MetricsDailyCacheID -> Integer,
        Total -> Integer,
        TotalSuccess -> Integer,
        TotalFailed -> Integer,
    }
}

diesel::table! {
    VstsFeatureCompliance (ID) {
        ID -> Integer,
        FeatureID -> Integer,
        ReleaseName -> Text,
        IsCompliant -> Bool,
        NumberNonCompliantChildren -> Integer,
        LastCheckedDate -> Timestamp,
    }
}

diesel::joinable!(ApplicationUser -> ApplicationUserRole (ApplicationUserRoleID));
diesel::joinable!(ApplicationUserReleaseApproval -> ApplicationUser (ApplicationUserID));
diesel::joinable!(ApplicationUserReleaseApproval -> ReleaseApprovalType (ReleaseApprovalTypeID));
diesel::joinable!(ApplicationUserTeam -> ApplicationTeam (ApplicationTeamID));
diesel::joinable!(ApplicationUserTeam -> ApplicationUser (ApplicationUserID));
diesel::joinable!(ReleaseActivity -> ApplicationTeam (ApplicationTeamID));
diesel::joinable!(ReleaseActivity -> Release (ReleaseID));
diesel::joinable!(ReleaseActivityApproval -> ApplicationUser (ApplicationUserID));
diesel::joinable!(ReleaseActivityApproval -> ReleaseActivity (ReleaseActivityID));
diesel::joinable!(ReleaseActivityApproval -> ReleaseApprovalType (ReleaseApprovalTypeID));
diesel::joinable!(ReleaseActivityFeature -> ReleaseActivity (ReleaseActivityID));
diesel::joinable!(ReleaseActivityRelatedTask -> ReleaseActivity (ReleaseActivityID));
diesel::joinable!(ReleaseActivityRelatedTask -> ReleaseActivityTask (ReleaseActivityTaskID));
diesel::joinable!(ReleaseActivityTask -> OctopusProject (OctopusProjectID));
diesel::joinable!(ReleaseActivityTaskAttachment -> ReleaseActivityTask (ReleaseActivityTaskID));
diesel::joinable!(ReleaseBranchHistory -> Release (ReleaseID));
diesel::joinable!(ReleaseRelatedCategory -> Release (ReleaseID));

diesel::allow_tables_to_appear_in_same_query!(
    ApplicationConnection,
    ApplicationNode,
    ApplicationTeam,
    ApplicationUser,
    ApplicationUserReleaseApproval,
    ApplicationUserReleaseAvailability,
    ApplicationUserRole,
    ApplicationUserTeam,
    AuditReleaseActivityTask,
    BuildHistory,
    BuildProject,
    Certificate,
    DeploymentHistory,
    ERTHistory,
    ERTProject,
    Environment,
    Export,
    FunctionalTest,
    FunctionalTestResult,
    MetricsBugs,
    MetricsDailyCache,
    MetricsPullRequests,
    NotificationQueue,
    OctopusProject,
    ProjectCodeCoverage,
    ProjectRollUp,
    ProjectRollUpOctopusProject,
    Release,
    ReleaseActivity,
    ReleaseActivityApproval,
    ReleaseActivityFeature,
    ReleaseActivityRelatedApplicationUser,
    ReleaseActivityRelatedTask,
    ReleaseActivityTask,
    ReleaseActivityTaskAttachment,
    ReleaseActivityTaskCategory,
    ReleaseActivityTaskMessageQueue,
    ReleaseActivityTaskStatus,
    ReleaseActivityTaskTargetEnvironment,
    ReleaseApproval,
    ReleaseApprovalStatistic,
    ReleaseApprovalType,
    ReleaseBranchHistory,
    ReleaseRelatedApplicationUser,
    ReleaseRelatedCategory,
    ReleaseStatus,
    RequestPathways,
    ResourceCost,
    ResourceGroupCost,
    STAR,
    STARData,
    SlottingMigrationLog,
    SystemEventLog,
    SystemValues,
    UnitTestHistory,
    VstsFeatureCompliance,
);
