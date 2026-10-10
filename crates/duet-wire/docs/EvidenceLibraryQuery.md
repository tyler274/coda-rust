# EvidenceLibraryQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anchor_record_id** | Option<**uuid::Uuid**> |  | [optional]
**categories** | Option<**Vec<Categories>**> |  (enum: medication, allergy, currentCondition, conditionOnRecord, pastCondition, lab, procedure, imaging, resultReport, clinicalDocument, encounter, immunization, coverage, visionPrescription, other) | [optional]
**coding_identities** | Option<[**Vec<models::ClinicalCodingIdentity>**](ClinicalCodingIdentity.md)> |  | [optional]
**cursor** | Option<**String**> |  | [optional]
**group_kind** | Option<**GroupKind**> |  (enum: medication, condition, allergy, immunization) | [optional]
**scope** | Option<**Scope**> |  (enum: summary, sourceSummary, completeRecord) | [optional][default to Summary]
**source_kind** | Option<**SourceKind**> |  (enum: appleHealth, healthRecords, epic, oracleHealth, healow, googleHealth, manual) | [optional]
**text** | Option<**String**> |  | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


