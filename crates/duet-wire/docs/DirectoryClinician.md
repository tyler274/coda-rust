# DirectoryClinician

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**npi** | **String** |  |
**kind** | **Kind** |  (enum: individual, organization) |
**name** | **String** |  |
**person** | Option<[**models::DirectoryPersonName**](DirectoryPersonName.md)> |  | [optional]
**credential** | Option<**String**> |  | [optional]
**primary_taxonomy** | Option<[**models::DirectoryTaxonomy**](DirectoryTaxonomy.md)> |  | [optional]
**taxonomies** | [**Vec<models::DirectoryTaxonomy>**](DirectoryTaxonomy.md) |  |
**practice_address** | Option<[**models::DirectoryAddress**](DirectoryAddress.md)> |  | [optional]
**mailing_address** | Option<[**models::DirectoryAddress**](DirectoryAddress.md)> |  | [optional]
**phone** | Option<**String**> |  | [optional]
**fax** | Option<**String**> |  | [optional]
**endpoints** | [**Vec<models::DirectoryEndpoint>**](DirectoryEndpoint.md) |  |
**last_updated** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


