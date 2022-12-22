import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://apq.ezrx.id/CMS/Account/Login?ReturnUrl=%2FCMS%2FHome')

WebUI.setText(findTestObject('null'), 'ace')

WebUI.setEncryptedText(findTestObject('null'), 'liSTkvaEvsTwAdgpFhbvKQ==')

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Information'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_ED Setting'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_ED Settings'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/input_Associated British F  B_dt-checkboxes'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/td_Set Date_dt-checkboxes-cell sorting_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_CMS APL/select_ON                                  _83d30e'), 
    '0', true)

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Set Date'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/input_Start OFF Date_startDateBulk'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/td_27'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/input_End OFF Date_endDateBulk'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/td_28'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Set'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Yes'))

WebUI.closeBrowser()

