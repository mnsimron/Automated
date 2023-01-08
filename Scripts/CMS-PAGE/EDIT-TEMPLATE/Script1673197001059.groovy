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

WebUI.click(findTestObject('Object Repository/Page_CMS APL/span_System Admin'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Template'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Lihat Details'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_Saatinianda terdaftar sebagai Customer _4297be'), '<span style="font-style: inherit; font-weight: inherit; -webkit-font-smoothing: antialiased; color: rgb(36, 36, 36); font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, system-ui, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Web&quot;, sans-serif; font-size: 12pt;">Saat</span><span style="font-style: inherit; font-weight: inherit; -webkit-font-smoothing: antialiased; color: rgb(36, 36, 36); font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, system-ui, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Web&quot;, sans-serif; font-size: 12pt;">&nbsp;</span><span style="font-style: inherit; font-weight: inherit; -webkit-font-smoothing: antialiased; color: rgb(36, 36, 36); font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, system-ui, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Web&quot;, sans-serif; font-size: 12pt;">ini</span><span style="font-style: inherit; font-weight: inherit; -webkit-font-smoothing: antialiased; color: rgb(36, 36, 36); font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, system-ui, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Web&quot;, sans-serif; font-size: 12pt;">&nbsp;anda terdaftar sebagai Customer General Trade dengan Jangkauan Produk<span style="-webkit-font-smoothing: antialiased;">&nbsp;</span><span style="-webkit-font-smoothing: antialiased; font-weight: bolder;">Consumer Goods</span></span>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Save'))

