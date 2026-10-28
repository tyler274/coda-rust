# CanonicalEvidenceChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity** | **Entity** |  (enum: clinical_note, health_evidence) |
**canonical_record_id** | **uuid::Uuid** |  |
**record_id** | **String** |  |
**revision** | **i64** |  |
**deleted** | **bool** |  |
**source_removed** | Option<**bool**> | Removed by its source; retained revisions remain citable. False for privacy erasure. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


