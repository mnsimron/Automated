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

WebUI.setText(findTestObject('Object Repository/Page_eZRX  Login page/input_Username_username'), 'ace')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_eZRX  Login page/input_Password_password'), 'liSTkvaEvsTwAdgpFhbvKQ==')

WebUI.click(findTestObject('Object Repository/Page_eZRX  Login page/button_Login'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_System Admin'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Template Order'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Edit'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/textarea_Toko Obatt'), 'Toko Obat')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Save'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/div_Description                            _680475'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/textarea_Toko Obatt'), '')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Save'))

WebUI.closeBrowser()

